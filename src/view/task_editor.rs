use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::model::task::Task;

pub struct TaskEditorState {
    task: Task,
}

pub struct TaskEditor {}

impl TaskEditor {
    pub fn new() -> Self {
        TaskEditor {}
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {}
            _ => {}
        }
        Ok(())
    }
}

impl StatefulWidget for &mut TaskEditor {
    type State = TaskEditorState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let text = vec![
            state.task.id.to_string().into(),
            Line::from(vec![
                state.task.name.clone().green().bold(),
                ": ".into(),
                state.task.description.clone().italic().gray(),
            ]),
        ];

        Paragraph::new(text).render(area, buf);
    }
}
