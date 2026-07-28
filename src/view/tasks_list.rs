use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{StatefulWidget, Widget},
};
use std::io::Result;
use tui_tree_widget::{Tree, TreeItem, TreeState};
use uuid::Uuid;

use crate::{
    model::{task::Task, tasks_manager::TasksManager},
    view::task_widget::TaskAppState,
};

pub struct TasksListState {
    pub tasks_manager: TasksManager,
    pub tree_state: TreeState<Uuid>,
}

impl TasksListState {
    pub fn new(tasks_manager: TasksManager) -> Self {
        TasksListState {
            tasks_manager,
            tree_state: TreeState::default(),
        }
    }
    pub fn get_selected_task(&mut self) -> &mut Task {
        let selected = self.tree_state.selected();
        return self
            .tasks_manager
            .get_task_mut(selected)
            .expect("no task selected");
    }

    fn track_selected(&mut self) -> bool {
        self.get_selected_task().track()
    }

    fn handle_mouse_event(&mut self, mouse_event: event::MouseEvent) -> Result<TaskAppState> {
        match mouse_event.kind {
            MouseEventKind::ScrollDown => self.tree_state.scroll_down(1),
            MouseEventKind::ScrollUp => self.tree_state.scroll_up(1),
            MouseEventKind::Down(_button) => self
                .tree_state
                .click_at(Position::new(mouse_event.column, mouse_event.row)),
            _ => false,
        };
        Ok(TaskAppState::ListView)
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<TaskAppState> {
        match key_event.code {
            KeyCode::Char('q') => return Ok(TaskAppState::Exiting),
            KeyCode::Char('e') => return Ok(TaskAppState::TaskEditView),
            KeyCode::Char('p' | ' ') => self.track_selected(),

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
        self.tasks_manager.save_tasks();
        Ok(TaskAppState::ListView)
    }

    pub fn handle_events(&mut self) -> Result<TaskAppState> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            _ => Ok(TaskAppState::ListView),
        }
    }
}

pub struct TasksList {}

impl<'a> TasksList {
    pub fn new() -> Self {
        Self {}
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

    fn tasks_tree(&self, task: &Task) -> Result<TreeItem<'a, Uuid>> {
        let mut items = Vec::new();
        for sub_task in &task.subtasks {
            let item = self.tasks_tree(sub_task)?;
            items.push(item);
        }
        return TreeItem::new(task.id, self.tasks_line(task), items);
    }

    fn items(&self, state: &TasksListState) -> Vec<TreeItem<'a, Uuid>> {
        let mut items = Vec::new();
        for task in state.tasks_manager.get_tasks() {
            let item = self.tasks_tree(task).expect("failed to create tree item");
            items.push(item);
        }
        return items;
    }
}

impl<'a> StatefulWidget for &'a mut TasksList {
    type State = TasksListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TasksListState) {
        let items = self.items(state);
        let tree = Tree::new(&items)
            .expect("duplicate tasks found")
            .highlight_style(
                Style::new().bg(Color::Black).fg(Color::Blue), // .add_modifier(Modifier::REVERSED),
            )
            .node_closed_symbol("> ")
            .node_open_symbol("v ");

        StatefulWidget::render(tree, area, buf, &mut state.tree_state)
    }
}
