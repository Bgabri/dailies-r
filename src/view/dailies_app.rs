use std::{io, time::Duration};

use crossterm::event;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{self, Line},
    widgets::{Block, Clear, StatefulWidget, Widget},
};
use ratatui_textarea::TextArea;
use uuid::Uuid;

use crate::model::{task::Task, tasks_manager::TasksManager};
use crate::view::{
    dailies_app::DailiesAppEvent::None,
    edit_task_view::EditTaskViewState,
    tasks_tree::{TasksTreeView, TasksTreeViewState},
};

pub enum DailiesAppState {
    ListView,
    EditTaskView(Vec<Uuid>, EditTaskViewState),
    AddTaskView(Vec<Uuid>, EditTaskViewState),
}

pub enum DailiesAppEvent {
    Exit,
    EditTask(Vec<Uuid>),
    AddTask(Vec<Uuid>),
    TrackTask(Vec<Uuid>),
    Save,
    Cancel,
    None,
}

pub struct DailiesApp {
    title: String,
    tasks_tree_state: TasksTreeViewState,
    state: DailiesAppState,
    tasks_manager: TasksManager,
    exit: bool,
}

impl DailiesApp {
    pub fn new(task_manager: TasksManager) -> Self {
        DailiesApp {
            title: "Task Manager".to_string(),
            tasks_tree_state: TasksTreeViewState::new(),
            state: DailiesAppState::ListView,
            tasks_manager: task_manager,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.handle_events()?;
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let frame_timeout = Duration::from_secs_f64(1.0 / 4.0);
        if !event::poll(frame_timeout)? {
            return Ok(());
        }

        let app_event: DailiesAppEvent = match &mut self.state {
            DailiesAppState::ListView => self.tasks_tree_state.handle_events()?,
            DailiesAppState::EditTaskView(_, task_editor_state) => {
                task_editor_state.handle_events()?
            }
            DailiesAppState::AddTaskView(_, task_editor_state) => {
                task_editor_state.handle_events()?
            }
            _ => None,
        };

        self.transition(app_event);
        Ok(())
    }

    fn transition(&mut self, event: DailiesAppEvent) {
        match event {
            DailiesAppEvent::Exit => self.exit(),
            DailiesAppEvent::EditTask(task_ids) => {
                let task = self.tasks_manager.get_task(&task_ids);

                if let Some(task) = task {
                    self.state = DailiesAppState::EditTaskView(
                        task_ids,
                        EditTaskViewState::new(task.clone()),
                    )
                }
            }
            DailiesAppEvent::AddTask(parent_ids) => {
                self.state =
                    DailiesAppState::AddTaskView(parent_ids, EditTaskViewState::new(Task::empty()));
            }

            DailiesAppEvent::TrackTask(task_ids) => {
                self.tasks_manager
                    .get_task_mut(&task_ids)
                    .and_then(|t| Some(t.track()));
            }

            DailiesAppEvent::Save => {
                if let DailiesAppState::EditTaskView(task_ids, task_editor_state) = &mut self.state
                {
                    let task = task_editor_state.get_task();
                    self.tasks_manager.update_task(task_ids, task);
                } else if let DailiesAppState::AddTaskView(parent_ids, task_editor_state) =
                    &mut self.state
                {
                    let task = task_editor_state.get_task();
                    self.tasks_manager.add_task(parent_ids, task);
                }
                self.state = DailiesAppState::ListView;
            }

            DailiesAppEvent::Cancel => self.state = DailiesAppState::ListView,
            DailiesAppEvent::None => {}
        }
    }

    fn render_edit_popup(
        task_editor_state: &mut EditTaskViewState,
        title: &str,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let mut task_editor = crate::view::edit_task_view::EditTaskView::new();
        let popup_block = Block::bordered()
            .title(Line::from(vec![
                title.into(),
                task_editor_state.get_task_id().to_string().dim().into(),
            ]))
            .border_set(border::ROUNDED);
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Length(4));

        Clear.render(centered_area, buf);
        task_editor.render(popup_block.inner(centered_area), buf, task_editor_state);
        popup_block.render(centered_area, buf);
    }
}

impl Widget for &mut DailiesApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let window = Block::bordered()
            .title(Line::from(self.title.clone()).centered())
            .border_set(border::ROUNDED);

        let tasks_list = TasksTreeView::new(self.tasks_manager.get_tasks());
        tasks_list.render(window.inner(area), buf, &mut self.tasks_tree_state);
        window.render(area, buf);

        match &mut self.state {
            DailiesAppState::AddTaskView(_, task_editor_state) => {
                DailiesApp::render_edit_popup(task_editor_state, "Add Task", area, buf);
            }
            DailiesAppState::EditTaskView(_, task_editor_state) => {
                DailiesApp::render_edit_popup(task_editor_state, "Edit Task", area, buf);
            }
            _ => {}
        }
    }
}
