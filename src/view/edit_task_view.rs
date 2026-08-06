use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{StatefulWidget, Widget},
};
use ratatui_textarea::TextArea;
use tui_tree_widget::Block;
use uuid::Uuid;

use crate::{model::task::Task, view::dailies_app::DailiesAppEvent};

#[derive(PartialEq, Eq)]
enum EditFocus {
    Name,
    Description,
    Association(usize),
    AddAssociation,
    None,
}

pub struct EditTaskViewState {
    task: Task,
    focus: EditFocus,
    //  TODO: remove static lifetime
    name_text_area: TextArea<'static>,
    description_text_area: TextArea<'static>,
    association_text_areas: Vec<TextArea<'static>>,
}

impl EditTaskViewState {
    pub fn new(task: Task) -> Self {
        let name_text_area = default_text_area(&task.name, "Enter name");
        let description_text_area = default_text_area(&task.description, "Enter description");
        let association_text_areas = task
            .associations
            .iter()
            .map(|a| default_text_area(a, "Enter association"))
            .collect();
        EditTaskViewState {
            task,
            focus: EditFocus::None,
            name_text_area,
            description_text_area,
            association_text_areas,
        }
    }

    fn get_focus_text_area_mut(&mut self) -> Option<&mut TextArea<'static>> {
        match self.focus {
            EditFocus::Name => Some(&mut self.name_text_area),
            EditFocus::Description => Some(&mut self.description_text_area),
            EditFocus::Association(i) => self.association_text_areas.get_mut(i),
            EditFocus::AddAssociation => None,
            EditFocus::None => None,
        }
    }

    fn switch_focus(&mut self, focus: EditFocus) {
        if let Some(text_area) = self.get_focus_text_area_mut() {
            inactivate_text_area(text_area);
        }
        self.focus = focus;
        if let Some(text_area) = self.get_focus_text_area_mut() {
            activate_text_area(text_area);
        }
    }

    fn focus_next(&mut self) {
        match self.focus {
            EditFocus::None => self.switch_focus(EditFocus::Name),
            EditFocus::Name => self.switch_focus(EditFocus::Description),
            EditFocus::Description => {
                if !self.association_text_areas.is_empty() {
                    self.switch_focus(EditFocus::Association(0));
                } else {
                    self.switch_focus(EditFocus::AddAssociation);
                }
            }
            EditFocus::Association(n) => {
                if n + 1 < self.association_text_areas.len() {
                    self.switch_focus(EditFocus::Association(n + 1));
                } else {
                    self.switch_focus(EditFocus::AddAssociation);
                }
            }
            EditFocus::AddAssociation => self.switch_focus(EditFocus::None),
        }
    }
    fn focus_prev(&mut self) {
        match self.focus {
            EditFocus::None => self.switch_focus(EditFocus::AddAssociation),
            EditFocus::Name => self.switch_focus(EditFocus::None),
            EditFocus::Description => self.switch_focus(EditFocus::Name),
            EditFocus::Association(n) => {
                if n > 0 {
                    self.switch_focus(EditFocus::Association(n - 1));
                } else {
                    self.switch_focus(EditFocus::Description);
                }
            }
            EditFocus::AddAssociation => {
                if !self.association_text_areas.is_empty() {
                    self.switch_focus(EditFocus::Association(
                        self.association_text_areas.len() - 1,
                    ));
                } else {
                    self.switch_focus(EditFocus::Description);
                }
            }
        }
    }
    fn update_values(&mut self) {
        self.task.name = self.name_text_area.lines().join("\n");
        self.task.description = self.description_text_area.lines().join("\n");
        self.task.associations = Vec::new();
        for association_text_area in &self.association_text_areas {
            let association = association_text_area.lines().join("\n");
            if !association.is_empty() {
                self.task.associations.push(association);
            }
        }
    }

    fn handle_text_event(&mut self, event: Event) {
        match self.focus {
            EditFocus::Name => {
                self.name_text_area.input(event);
            }
            EditFocus::Description => {
                self.description_text_area.input(event);
            }
            EditFocus::Association(i) => {
                if let Some(text_area) = self.association_text_areas.get_mut(i) {
                    text_area.input(event);
                }
            }
            EditFocus::AddAssociation => {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char(' ' | 'a' | 'A'),
                    ..
                }) = event
                {
                    self.association_text_areas
                        .push(default_text_area("", "Enter association"));
                    self.switch_focus(EditFocus::Association(
                        self.association_text_areas.len() - 1,
                    ));
                }
            }
            EditFocus::None => {}
        }
    }

    pub fn handle_events(&mut self, event: Event) -> DailiesAppEvent {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return DailiesAppEvent::Cancel,

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                self.update_values();
                return DailiesAppEvent::Save;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab | KeyCode::Down,
                ..
            }) => self.focus_next(),
            Event::Key(KeyEvent {
                code: KeyCode::BackTab | KeyCode::Up,
                ..
            }) => self.focus_prev(),

            event => self.handle_text_event(event),
        }
        DailiesAppEvent::None
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
            .constraints(vec![Constraint::Length(1); 3])
            .split(area);

        state.name_text_area.render(layout[0], buf);
        state.description_text_area.render(layout[1], buf);

        let association_layout = Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(5);
                state.association_text_areas.len() + 1
            ])
            .split(layout[2]);
        let mut add_association = Span::from(" [+] ");
        if state.focus == EditFocus::AddAssociation {
            add_association =
                add_association.style(Style::default().bg(Color::Black).fg(Color::Blue));
        }
        add_association.render(association_layout[association_layout.len() - 1], buf);

        for (i, association_text_area) in state.association_text_areas.iter_mut().enumerate() {
            association_text_area.render(association_layout[i], buf);
        }
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
