use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::model::{task::Task, tasks_manager::TasksManager};
use crate::view::tasks_list::TasksList;


pub struct TaskApp<'a> {
    title: String,
    // task_manager: &'a mut TasksManager,
    exit: bool,
    tasks_list: TasksList<'a>,
}

impl<'a> TaskApp<'a> {
    pub fn new(task_manager: &'a mut TasksManager) -> Self {
        TaskApp {
            title: "Task Manager".to_string(),
            // task_manager,
            tasks_list: TasksList::new(task_manager),
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
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

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    self.exit();
                }
                self.tasks_list.handle_key_event(key_event)
            }
            Event::Mouse(mouse_event) => self.tasks_list.handle_mouse_event(mouse_event),
            _ => {}
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
    }
}
