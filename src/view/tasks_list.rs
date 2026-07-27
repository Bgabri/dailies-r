use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};
use uuid::Uuid;

use crate::model::{task::Task, tasks_manager::TasksManager};

pub struct TasksList<'a> {
    tasks_manager: &'a mut TasksManager,
    pub state: TreeState<Uuid>,
}

impl<'a> TasksList<'a> {
    pub fn new(tasks_manager: &'a mut TasksManager) -> Self {
        Self {
            state: TreeState::default(),
            tasks_manager,
        }
    }

    fn tasks_line(&self, task: &Task) -> Line<'a> {
        let mut line: Line;
        if task.in_session() {
            let elapsed_time = task.elapsed_session_time();

            let hour = elapsed_time.num_hours();
            let minute = elapsed_time.num_minutes() % 60;
            let second = elapsed_time.num_seconds() % 60;

            line = Line::from(vec![
                task.name.clone().green().bold(),
                ": ".into(),
                task.description.clone().italic().gray(),
                format!(" {}:{:02}:{:02}", hour, minute, second)
                    .green()
                    .bold(),
            ]);
        } else {
            line = Line::from(vec![
                task.name.clone().bold(),
                ": ".into(),
                task.description.clone().gray().italic(),
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

    fn tasks_tree(&self, task: &Task) -> Result<TreeItem<'a, Uuid>, std::io::Error> {
        let mut items = Vec::new();
        for sub_task in &task.subtasks {
            let item = self.tasks_tree(sub_task)?;
            items.push(item);
        }
        return TreeItem::new(task.id, self.tasks_line(task), items);
    }

    fn items(&self) -> Vec<TreeItem<'a, Uuid>> {
        let mut items = Vec::new();
        for task in self.tasks_manager.get_tasks() {
            let item = self.tasks_tree(task).expect("failed to create tree item");
            items.push(item);
        }
        return items;
    }

    fn get_selected_task(&mut self) -> &mut Task {
        let selected = self.state.selected();
        return self
            .tasks_manager
            .get_task_mut(selected)
            .expect("no task selected");
    }

    fn track_selected(&mut self) -> bool {
        self.get_selected_task().track()
    }

    fn edit_selected(&mut self) -> bool {
        // self.get_selected_task().edit()
        false
    }

    pub fn handle_mouse_event(&mut self, mouse_event: event::MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::ScrollDown => self.state.scroll_down(1),
            MouseEventKind::ScrollUp => self.state.scroll_up(1),
            MouseEventKind::Down(_button) => self
                .state
                .click_at(Position::new(mouse_event.column, mouse_event.row)),
            _ => false,
        };
    }

    pub fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('p' | ' ') => self.track_selected(),
            KeyCode::Char('e') => self.edit_selected(),
            // navigation
            KeyCode::Enter | KeyCode::Char('\n') => self.state.toggle_selected(),
            KeyCode::Left => self.state.key_left(),
            KeyCode::Right => self.state.key_right(),
            KeyCode::Down => self.state.key_down(),
            KeyCode::Up => self.state.key_up(),
            KeyCode::Esc => self.state.select(Vec::new()),
            KeyCode::Home => self.state.select_first(),
            KeyCode::End => self.state.select_last(),
            KeyCode::PageDown => self.state.scroll_down(3),
            KeyCode::PageUp => self.state.scroll_up(3),
            _ => false,
        };
    }
}

impl Widget for &mut TasksList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.items();
        let tree = Tree::new(&items)
            .expect("duplicate tasks found")
            .highlight_style(
                Style::new().bg(Color::Black).fg(Color::Blue), // .add_modifier(Modifier::REVERSED),
            )
            .node_closed_symbol("> ")
            .node_open_symbol("v ");

        StatefulWidget::render(tree, area, buf, &mut self.state)
    }
}
