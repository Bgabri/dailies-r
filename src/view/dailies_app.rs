use std::{io, time::Duration};

use crossterm::event;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Clear, StatefulWidget, Widget},
};

use crate::view::{
    dailies_app::DailiesAppTransition::None, task_editor::TaskEditorState, tasks_list::TasksList,
};
use crate::{
    model::{task::Task, tasks_manager::TasksManager},
    view::tasks_list::TasksListState,
};

#[derive(PartialEq, Eq)]
pub enum DailiesAppState {
    ListView,
    TaskEditView(TaskEditorState),
    TaskAddView,
    Exited,
}

pub enum DailiesAppTransition {
    Exit,
    EditTask(Task),
    Save,
    Cancel,
    None,
}

pub struct DailiesApp {
    title: String,
    tasks_list_state: TasksListState,
    state: DailiesAppState,
}

impl DailiesApp {
    pub fn new(task_manager: TasksManager) -> Self {
        DailiesApp {
            title: "Task Manager".to_string(),
            tasks_list_state: TasksListState::new(task_manager),
            state: DailiesAppState::ListView,
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

        let app_event: DailiesAppTransition = match &mut self.state {
            DailiesAppState::ListView => self.tasks_list_state.handle_events()?,

            DailiesAppState::TaskEditView(task_editor_state) => {
                task_editor_state.handle_events()?
            }
            _ => None,
        };

        match app_event {
            DailiesAppTransition::Exit => self.state = DailiesAppState::Exited,
            DailiesAppTransition::EditTask(task) => {
                self.state = DailiesAppState::TaskEditView(TaskEditorState::new(task))
            }
            DailiesAppTransition::Cancel => self.state = DailiesAppState::ListView,
            DailiesAppTransition::Save => {}
            DailiesAppTransition::None => {}
        }

        Ok(())
    }
}

impl Widget for &mut DailiesApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let window = Block::bordered()
            .title(Line::from(self.title.clone()).centered())
            .border_set(border::ROUNDED);

        let mut tasks_list: TasksList = TasksList::new();
        tasks_list.render(window.inner(area), buf, &mut self.tasks_list_state);
        window.render(area, buf);

        if let DailiesAppState::TaskEditView(task_editor_state) = &mut self.state {
            let mut task_editor = crate::view::task_editor::TaskEditor::new();
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
