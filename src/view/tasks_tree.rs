use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::StatefulWidget,
};
use std::io::Result;
use tui_tree_widget::{Tree, TreeItem, TreeState};
use uuid::Uuid;

use crate::{model::task::Task, view::dailies_app::DailiesAppEvent};

pub struct TasksTreeViewState {
    tree_state: TreeState<Uuid>,
}

impl<'a> TasksTreeViewState {
    pub fn new() -> Self {
        TasksTreeViewState {
            tree_state: TreeState::default(),
        }
    }

    fn get_selected(&mut self) -> Vec<Uuid> {
        self.tree_state.selected().to_vec()
    }

    fn handle_mouse_event(&mut self, mouse_event: event::MouseEvent) -> DailiesAppEvent {
        match mouse_event.kind {
            MouseEventKind::ScrollDown => self.tree_state.scroll_down(1),
            MouseEventKind::ScrollUp => self.tree_state.scroll_up(1),
            MouseEventKind::Down(_button) => self
                .tree_state
                .click_at(Position::new(mouse_event.column, mouse_event.row)),
            _ => false,
        };
        DailiesAppEvent::None
    }

    fn handle_key_press(&mut self, key_event: event::KeyEvent) -> DailiesAppEvent {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => return DailiesAppEvent::Exit,
            KeyCode::Char('p' | 'P' | ' ') => {
                return DailiesAppEvent::TrackTask(self.get_selected());
            }
            KeyCode::Char('e' | 'E') => {
                return DailiesAppEvent::EditTask(self.get_selected());
            }
            KeyCode::Char('a' | 'A') => {
                return DailiesAppEvent::AddTask(self.get_selected());
            }
            KeyCode::Char('d' | 'D') => {
                return DailiesAppEvent::DeleteTask(self.get_selected());
            }

            // navigation
            KeyCode::Enter | KeyCode::Char('\n') => self.tree_state.toggle_selected(),
            KeyCode::Left => self.tree_state.key_left(),
            KeyCode::Right => self.tree_state.key_right(),
            KeyCode::Down => self.tree_state.key_down(),
            KeyCode::Up => self.tree_state.key_up(),
            KeyCode::Esc => self.tree_state.select(Vec::new()),
            KeyCode::Home => self.tree_state.select_first(),
            KeyCode::End => self.tree_state.select_last(),
            KeyCode::PageDown => self.tree_state.scroll_down(3),
            KeyCode::PageUp => self.tree_state.scroll_up(3),
            _ => false,
        };
        DailiesAppEvent::None
    }

    pub fn handle_events(&mut self) -> Result<DailiesAppEvent> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(self.handle_key_press(key_event))
            }
            Event::Mouse(mouse_event) => Ok(self.handle_mouse_event(mouse_event)),
            _ => Ok(DailiesAppEvent::None),
        }
    }
}

pub struct TasksTreeView<'a> {
    items: Vec<TreeItem<'a, Uuid>>,
}

impl<'a> TasksTreeView<'a> {
    pub fn new(tasks: &'a [Task]) -> Self {
        Self {
            items: Self::items(tasks),
        }
    }
    fn tasks_line(task: &Task) -> Line<'_> {
        let mut line: Line;
        if task.in_session() {
            let elapsed_time = task.elapsed_session_time();

            let hour = elapsed_time.num_hours();
            let minute = elapsed_time.num_minutes() % 60;
            let second = elapsed_time.num_seconds() % 60;

            line = Line::from(vec![
                task.name.clone().green().bold(),
                ": ".into(),
                task.description.clone().gray(),
                format!(" {}:{:02}:{:02}", hour, minute, second)
                    .green()
                    .bold(),
            ]);
        } else {
            line = Line::from(vec![
                task.name.clone().bold(),
                ": ".into(),
                task.description.clone().gray(),
            ]);
        }

        if task.is_completed() {
            line = line.style(Style::new().dim());
            for span in &mut line.spans {
                span.style = span.style.crossed_out();
            }
        }

        line
    }

    fn tasks_tree(task: &Task) -> Result<TreeItem<'_, Uuid>> {
        let mut items = Vec::new();
        for sub_task in &task.subtasks {
            let item = Self::tasks_tree(sub_task)?;
            items.push(item);
        }
        return TreeItem::new(task.id, Self::tasks_line(task), items);
    }

    fn items(tasks: &[Task]) -> Vec<TreeItem<'_, Uuid>> {
        let mut items = Vec::new();
        for task in tasks {
            let item = Self::tasks_tree(task).expect("failed to create tree item");
            items.push(item);
        }
        return items;
    }
}

impl StatefulWidget for &TasksTreeView<'_> {
    type State = TasksTreeViewState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TasksTreeViewState) {
        let tree = Tree::new(&self.items)
            .expect("duplicate tasks found")
            .highlight_style(
                Style::new().bg(Color::Black).fg(Color::Blue), // .add_modifier(Modifier::REVERSED),
            )
            .node_closed_symbol("> ")
            .node_open_symbol("v ");

        StatefulWidget::render(tree, area, buf, &mut state.tree_state)
    }
}
