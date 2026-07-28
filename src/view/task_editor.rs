use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::model::task::Task;

pub struct TaskEditor<'a> {
    task: &'a mut Task,
}

impl<'a> TaskEditor<'a> {
    pub fn new(task: &'a mut Task) -> Self {
        TaskEditor { task }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {}
            _ => {}
        }
        Ok(())
    }
}

impl Widget for &mut TaskEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = vec![
            self.task.id.to_string().into(),
            Line::from(vec![
                self.task.name.clone().green().bold(),
                ": ".into(),
                self.task.description.clone().italic().gray(),
            ]),
        ];

        Paragraph::new(text).render(area, buf);
    }
}
