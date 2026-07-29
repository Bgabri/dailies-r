use std::{io, time::Duration};

use crossterm::event;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect},
    symbols::border,
    text::Line,
    widgets::{Block, Clear, StatefulWidget, Widget},
};
use uuid::Uuid;

use crate::model::{task::Task, tasks_manager::TasksManager};
use crate::view::{
    dailies_app::DailiesAppEvent::None,
    edit_task_view::EditTaskViewState,
    tasks_tree::{TasksTreeView, TasksTreeViewState},
};

#[derive(PartialEq, Eq)]
pub enum DailiesAppState {
    ListView,
    EditTaskView(EditTaskViewState),
    AddTaskView(Vec<Uuid>, EditTaskViewState),
    Exited,
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
}

impl DailiesApp {
    pub fn new(task_manager: TasksManager) -> Self {
        DailiesApp {
            title: "Task Manager".to_string(),
            tasks_tree_state: TasksTreeViewState::new(),
            state: DailiesAppState::ListView,
            tasks_manager: task_manager,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state != DailiesAppState::Exited {
            self.handle_events()?;
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
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
            DailiesAppState::EditTaskView(task_editor_state) => {
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
            DailiesAppEvent::Exit => self.state = DailiesAppState::Exited,
            DailiesAppEvent::EditTask(task_ids) => {
                let task = self.tasks_manager.get_task(&task_ids);

                if let Some(task) = task {
                    self.state = DailiesAppState::EditTaskView(EditTaskViewState::new(task.clone()))
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

            DailiesAppEvent::Cancel => self.state = DailiesAppState::ListView,
            DailiesAppEvent::Save => {}
            DailiesAppEvent::None => {}
        }
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

        if let DailiesAppState::EditTaskView(task_editor_state) = &mut self.state {
            let mut task_editor = crate::view::edit_task_view::EditTaskView::new();
            let popup_block = Block::bordered()
                .title("Editing")
                .border_set(border::ROUNDED);
            let centered_area =
                area.centered(Constraint::Percentage(60), Constraint::Percentage(20));

            Clear.render(centered_area, buf);
            task_editor.render(popup_block.inner(centered_area), buf, task_editor_state);
            popup_block.render(centered_area, buf);
        }
    }
}
