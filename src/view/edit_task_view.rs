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
    //  TODO: remove static lifetime
    name_text_area: TextArea<'static>,
    description_text_area: TextArea<'static>,
}

impl EditTaskViewState {
    pub fn new(task: Task) -> Self {
        let name_text_area = default_text_area(&task.name, "Enter name");
        let description_text_area = default_text_area(&task.description, "Enter description");
        EditTaskViewState {
            task,
            focus: EditFocus::None,
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

    fn focus_next(&mut self) {
        match self.focus {
            EditFocus::None => self.switch_focus(EditFocus::Name),
            EditFocus::Name => self.switch_focus(EditFocus::Description),
            EditFocus::Description => self.switch_focus(EditFocus::None),
        }
    }
    fn focus_prev(&mut self) {
        match self.focus {
            EditFocus::None => self.switch_focus(EditFocus::Description),
            EditFocus::Name => self.switch_focus(EditFocus::None),
            EditFocus::Description => self.switch_focus(EditFocus::Name),
        }
    }
    fn update_values(&mut self) {
        self.task.name = self.name_text_area.lines().join("\n");
        self.task.description = self.description_text_area.lines().join("\n");
    }

    pub fn handle_events(&mut self) -> std::io::Result<DailiesAppEvent> {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return Ok(DailiesAppEvent::Cancel),

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                self.update_values();
                return Ok(DailiesAppEvent::Save);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab | KeyCode::Down,
                ..
            }) => self.focus_next(),
            Event::Key(KeyEvent {
                code: KeyCode::BackTab | KeyCode::Up,
                ..
            }) => self.focus_prev(),

            event => match self.focus {
                EditFocus::Name => {
                    self.name_text_area.input(event);
                }
                EditFocus::Description => {
                    self.description_text_area.input(event);
                }
                _ => {}
            },
        }
        Ok(DailiesAppEvent::None)
    }

    pub fn get_task(&self) -> Task {
        self.task.clone()
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
    textarea.set_placeholder_style(Style::default().dim().italic());
    inactivate_text_area(&mut textarea);
    textarea
}
fn inactivate_text_area(textarea: &mut TextArea<'_>) {
    textarea.set_style(Style::default());
    textarea.set_block(Block::default());
    textarea.set_selection_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_cursor_line_style(Style::default());
}

fn activate_text_area(textarea: &mut TextArea<'_>) {
    textarea.set_selection_style(Style::default().reversed());
    textarea.set_block(Block::default().bg(Color::Black).fg(Color::Blue));
    textarea.set_cursor_style(Style::default().fg(Color::Reset).reversed());
}
