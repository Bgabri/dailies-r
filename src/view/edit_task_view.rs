use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::{model::task::Task, view::dailies_app::DailiesAppEvent};

#[derive(PartialEq, Eq)]
pub struct EditTaskViewState {
    task: Task,
}

impl EditTaskViewState {
    pub fn new(task: Task) -> Self {
        EditTaskViewState { task }
    }

    fn handle_key_press(&mut self, key_code: KeyCode) -> DailiesAppEvent {
        match key_code {
            KeyCode::Esc => DailiesAppEvent::Cancel,
            _ => DailiesAppEvent::None,
        }
    }

    pub fn handle_events(&mut self) -> std::io::Result<DailiesAppEvent> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(self.handle_key_press(key_event.code))
            }
            _ => Ok(DailiesAppEvent::None),
        }
    }
}

pub struct EditTaskView {}

impl EditTaskView {
    pub fn new() -> Self {
        EditTaskView {}
    }
}

impl StatefulWidget for & mut EditTaskView {
    type State = EditTaskViewState;
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
