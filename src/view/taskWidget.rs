use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Modifier,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, Widget},
};

use crate::model::task::Task;

pub struct TaskWidget<'a> {
    task: &'a Task,
}

impl<'a> TaskWidget<'a> {
    pub fn new(task: &'a Task) -> Self {
        TaskWidget { task }
    }
}

impl<'a> Widget for TaskWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let name = Span::styled(self.task.name.clone(), Modifier::BOLD);
        let description = Span::raw(&self.task.description);
        let line = Line::from(vec![name, description]);
        line.render(area, buf);
    }
}

pub struct TaskApp {
    title: String,
    tasks: Vec<Task>,
    exit: bool,
}

impl TaskApp {
    pub fn new() -> Self {
        let tasks = vec![
            Task::new("Task 1".to_string(), "Description for Task 1".to_string()),
            Task::new("Task 2".to_string(), "Description for Task 2".to_string()),
        ];
        TaskApp {
            title: "Task App".to_string(),
            tasks,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // todo!()
        Ok(())
    }
}

impl Widget for &TaskApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let window = Block::bordered()
            .title(Line::from(self.title.clone()).centered())
            .border_set(border::ROUNDED);


        let constraints = vec![Constraint::Length(1); self.tasks.len()];

        let list = Layout::vertical(constraints).flex(Flex::Start).split(area);

        for (i, cell) in list.iter().enumerate() {
            let task = &self.tasks[i];
            let task_widget = TaskWidget::new(task);
            task_widget.render(*cell, buf);
        }
    }
}
