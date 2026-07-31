use std::f32::consts::E;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{self, Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Padding, Paragraph, StatefulWidget, Widget, Wrap},
};
use ratatui_textarea::{Input, Key, TextArea};
use std::io::Result;
use tui_tree_widget::Block;
use uuid::Uuid;

use crate::{model::task::Task, view::dailies_app::DailiesAppEvent};

#[derive(Debug, Clone)]
pub struct ConfirmViewState<'a> {
    pub message: Line<'a>,
}

impl<'a> ConfirmViewState<'a> {
    pub fn new(message: Line<'a>) -> Self {
        ConfirmViewState { message }
    }
    pub fn handle_events(&mut self) -> Result<DailiesAppEvent> {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('y' | 'Y') | KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(DailiesAppEvent::Save),
            Event::Key(KeyEvent {
                code: KeyCode::Char('n' | 'N') | KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) => Ok(DailiesAppEvent::Cancel),
            _ => Ok(DailiesAppEvent::None),
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
