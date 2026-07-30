use std::f32::consts::E;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Paragraph, StatefulWidget, Widget},
};
use ratatui_textarea::{Input, Key, TextArea};
use tui_tree_widget::Block;
use uuid::Uuid;

use crate::{model::task::Task, view::dailies_app::DailiesAppEvent};

enum EditFocus {
    Name,
    Description,
    None,
}

pub struct EditTaskViewState {
    task: Task,
    focus: EditFocus,
    name_text_area: TextArea<'static>,
    description_text_area: TextArea<'static>,
}

impl EditTaskViewState {
    pub fn new(task: Task) -> Self {
        let name_text_area = default_text_area(&task.name, "Enter name");

        let description_text_area = default_text_area(&task.description, "Enter description");
        EditTaskViewState {
            task,
            focus: EditFocus::Name,
            name_text_area,
            description_text_area,
        }
    }

    fn switch_focus(&mut self, focus: EditFocus) {
        match focus {
            EditFocus::Name => {
                activate_text_area(&mut self.name_text_area);
                inactivate_text_area(&mut self.description_text_area);
            }
            EditFocus::Description => {
                activate_text_area(&mut self.description_text_area);
                inactivate_text_area(&mut self.name_text_area);
            }
            EditFocus::None => {
                inactivate_text_area(&mut self.name_text_area);
                inactivate_text_area(&mut self.description_text_area);
            }
        }
        self.focus = focus;
    }

    fn handle_name_event(&mut self, key_event: Event) {
        match key_event {
            Event::Key(KeyEvent {
                code: KeyCode::Tab | KeyCode::BackTab,
                modifiers: modifier,
                ..
            }) => {
                self.switch_focus(EditFocus::Description);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {}
            _ => {
                self.name_text_area.input(key_event);
            }
        }
    }

    fn handle_description_event(&mut self, key_event: Event) {
        match key_event {
            Event::Key(KeyEvent {
                code: KeyCode::Tab | KeyCode::BackTab,
                ..
            }) => {
                self.switch_focus(EditFocus::Name);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {}
            _ => {
                self.description_text_area.input(key_event);
            }
        }
    }

    fn handle_none_event(&mut self, key_event: Event) {
        match key_event {
            Event::Key(KeyEvent {
                code: KeyCode::Tab, ..
            }) => {
                self.focus = EditFocus::Name;
            }
            _ => {}
        }
    }

    pub fn handle_events(&mut self) -> std::io::Result<DailiesAppEvent> {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return Ok(DailiesAppEvent::Cancel),
            event => match self.focus {
                EditFocus::Name => self.handle_name_event(event),
                EditFocus::Description => self.handle_description_event(event),
                EditFocus::None => self.handle_none_event(event),
            },
        }
        Ok(DailiesAppEvent::None)
    }

    pub fn get_task_id(&self) -> Uuid {
        self.task.id
    }
}

pub struct EditTaskView {}

impl EditTaskView {
    pub fn new() -> Self {
        EditTaskView {}
    }
}

impl StatefulWidget for &mut EditTaskView {
    type State = EditTaskViewState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Length(1)])
            .split(area);

        state.name_text_area.render(layout[0], buf);
        state.description_text_area.render(layout[1], buf);
    }
}

fn default_text_area(value: &str, place_holder: &str) -> TextArea<'static> {
    let mut textarea = TextArea::from(vec![value]);
    textarea.set_placeholder_text(place_holder);
    textarea.set_placeholder_style(
        Style::default()
            .add_modifier(Modifier::DIM)
            .add_modifier(Modifier::ITALIC),
    );
    inactivate_text_area(&mut textarea);
    textarea
}
fn inactivate_text_area(textarea: &mut TextArea<'_>) {
    textarea.set_style(Style::default());
    textarea.set_cursor_line_style(Style::default());
    textarea.set_block(Block::default());
    textarea.set_cursor_style(Style::default());
}

fn activate_text_area(textarea: &mut TextArea<'_>) {
    textarea.set_block(Block::default().bg(Color::Black).fg(Color::Blue));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
}
