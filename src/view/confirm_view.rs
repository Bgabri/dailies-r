use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_tree_widget::Block;

use crate::view::dailies_app::DailiesAppEvent;

#[derive(Debug, Clone)]
pub struct ConfirmViewState<'a> {
    pub message: Line<'a>,
}

impl<'a> ConfirmViewState<'a> {
    pub fn new(message: Line<'a>) -> Self {
        ConfirmViewState { message }
    }
    pub fn handle_events(&mut self, event: Event) -> DailiesAppEvent {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('y' | 'Y') | KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => DailiesAppEvent::Save,
            Event::Key(KeyEvent {
                code: KeyCode::Char('n' | 'N') | KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) => DailiesAppEvent::Cancel,
            _ => DailiesAppEvent::None,
        }
    }
}

pub struct ConfirmView {}

impl ConfirmView {
    pub fn new() -> Self {
        ConfirmView {}
    }
}

impl<'a> StatefulWidget for &'a ConfirmView {
    type State = ConfirmViewState<'a>;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .title("!".bold().red())
            .border_set(border::ROUNDED);

        let message_paragraph = Paragraph::new(state.message.clone())
            .red()
            .wrap(Wrap::default())
            .centered();

        message_paragraph.render(block.inner(area), buf);
        block.render(area, buf);
    }
}
