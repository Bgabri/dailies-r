use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::StatefulWidget,
};
use std::io::Result;
use tui_tree_widget::{Tree, TreeItem, TreeState};
use uuid::Uuid;

use crate::{model::task::Task, view::task_manager_view::ManagerViewEvent};

pub struct TasksTreeViewState {
    tree_state: TreeState<Uuid>,
}

impl<'a> TasksTreeViewState {
    pub fn new() -> Self {
        TasksTreeViewState {
            tree_state: TreeState::default(),
        }
    }

    pub fn open_all(&mut self, tasks: &[Task]) {
        let mut tree_state = TreeState::default();
        let mut path = Vec::new();

        Self::open_all_rec(&mut tree_state, tasks, &mut path);
        self.tree_state = tree_state;
    }

    fn open_all_rec(state: &mut TreeState<Uuid>, tasks: &[Task], path: &mut Vec<Uuid>) {
        for task in tasks {
            path.push(task.id);

            if !task.subtasks.is_empty() {
                state.open(path.clone());
                Self::open_all_rec(state, &task.subtasks, path);
            }

            path.pop();
        }
    }

    fn get_selected(&mut self) -> Vec<Uuid> {
        self.tree_state.selected().to_vec()
    }

    fn handle_key_press(&mut self, key_event: event::KeyEvent) -> ManagerViewEvent {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => return ManagerViewEvent::Exit,
            KeyCode::Char('p' | 'P' | ' ') => {
                return ManagerViewEvent::TrackTask(self.get_selected());
            }
            KeyCode::Char('e' | 'E') => {
                return ManagerViewEvent::EditTask(self.get_selected());
            }
            KeyCode::Char('a' | 'A') => {
                return ManagerViewEvent::AddTask(self.get_selected());
            }
            KeyCode::Char('d' | 'D') => {
                return ManagerViewEvent::DeleteTask(self.get_selected());
            }
            KeyCode::Char('c' | 'C') => {
                return ManagerViewEvent::CompleteTask(self.get_selected());
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
        ManagerViewEvent::None
    }

    pub fn handle_events(&mut self, event: Event) -> ManagerViewEvent {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_press(key_event)
            }
            _ => ManagerViewEvent::None,
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
            let elapsed_time = task.elapsed_daily_time();

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
