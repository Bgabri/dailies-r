use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Position, Rect}, style::{Color, Modifier, Style, Stylize}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Clear, StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::model::{task::Task, tasks_manager::TasksManager};
use crate::view::tasks_list::TasksList;

#[derive(PartialEq, Eq)]
pub enum TaskAppState {
    ListView,
    TaskEditView,
    TaskAddView,
    Exiting,
}

pub struct TaskApp<'a> {
    title: String,
    tasks_list: TasksList<'a>,
    state: TaskAppState,
}

impl<'a> TaskApp<'a> {
    pub fn new(task_manager: &'a mut TasksManager) -> Self {
        TaskApp {
            title: "Task Manager".to_string(),
            // task_manager,
            tasks_list: TasksList::new(task_manager),
            state: TaskAppState::ListView,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state != TaskAppState::Exiting {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
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

        match self.state {
            TaskAppState::ListView => {
                self.state = self.tasks_list.handle_events()?;
            }

            // TaskAppState::TaskEditView => {
            //     self.state = self.
            // }
            _ => {
                self.state = TaskAppState::ListView;
            }
        }

        Ok(())
    }
}

impl<'a> Widget for &mut TaskApp<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let window = Block::bordered()
            .title(Line::from(self.title.clone()).centered())
            .border_set(border::ROUNDED);

        self.tasks_list.render(window.inner(area), buf);
        window.render(area, buf);

        if self.state == TaskAppState::TaskEditView {
            let mut task_editor =
                crate::view::task_editor::TaskEditor::new(self.tasks_list.get_selected_task());
            let popup_block = Block::bordered().title("Editing").border_set(border::ROUNDED);
            let centered_area =
                area.centered(Constraint::Percentage(60), Constraint::Percentage(20));

            Clear.render(centered_area, buf);
            task_editor.render(popup_block.inner(centered_area), buf);
            popup_block.render(centered_area, buf);

        }
        
    }
}