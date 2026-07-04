use core::time::Duration;
use std::io;
use std::process::id;
use std::time::Instant;

use crossterm::event;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Scrollbar, ScrollbarOrientation, Widget};
use ratatui::{DefaultTerminal, Frame, Terminal, crossterm};
use tui_tree_widget::{Tree, TreeItem, TreeState};

mod model;
use model::task::Task;
use uuid::Uuid;

#[must_use]
struct App<'a> {
    state: TreeState<&'a Task>,
    items: Vec<TreeItem<'a, &'a Task>>,
    exit: bool,

    dbg_hud: String,
}

impl<'a> App<'a> {
    fn tasks_tree(task: &Task) -> Result<TreeItem<'_, &Task>, std::io::Error> {
        if task.subtasks.len() == 0 {
            return Ok(TreeItem::new_leaf(&task, task.name.clone()));
        }

        let mut items = Vec::new();
        for sub_task in &task.subtasks {
            let item = App::tasks_tree(sub_task)?;
            items.push(item);
        }
        return TreeItem::new(&task, task.name.clone(), items);
    }

    fn new(tasks: &'a [Task]) -> Self {
        let mut items: Vec<TreeItem<'a, &'a Task>> = Vec::new();
        for task in tasks {
            let item = App::tasks_tree(task).expect("duplicate tasks found");
            items.push(item);
        }

        Self {
            state: TreeState::default(),
            items,
            exit: false,
            dbg_hud: String::new(),
        }
    }

    fn exit(&mut self) -> bool {
        self.exit = true;
        self.exit
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let tree = Tree::new(&self.items)
            .expect("duplicate tasks found")
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .node_closed_symbol("> ")
            .node_open_symbol("v ");
        frame.render_stateful_widget(tree, area, &mut self.state);

        let span = Span::raw(&self.dbg_hud);

        let line = Line::from(span).right_aligned();

        let buf = frame.buffer_mut();
        line.render(area, buf);
        // frame.render_widget(spa, area);
        // span.render(area, frame.buffer_mut());
        // frame.area().;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if self.state.selected().len() > 0 {
            self.dbg_hud = format!(
                "{:?}",
                self.state.selected().last().expect("no tree item selected")
            );
        }
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: event::MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::ScrollDown => self.state.scroll_down(1),
            MouseEventKind::ScrollUp => self.state.scroll_up(1),
            MouseEventKind::Down(_button) => self
                .state
                .click_at(Position::new(mouse_event.column, mouse_event.row)),
            _ => false,
        };
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter | KeyCode::Char('\n' | ' ') => self.state.toggle_selected(),
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

fn main() {
    let mut task = Task::new("My Task".to_string(), "This is a sample task.".to_string());
    let mut subtask1 = Task::new(
        "Subtask 1".to_string(),
        "This is the first subtask.".to_string(),
    );
    let subtask2 = Task::new(
        "Subtask 2".to_string(),
        "This is the second subtask.".to_string(),
    );
    let subsubtask = Task::new(
        "Sub-subtask".to_string(),
        "This is a sub-subtask.".to_string(),
    );
    subtask1.add_subtask(subsubtask);
    task.add_subtask(subtask1);
    task.add_subtask(subtask2);
    let tasks = vec![task];
    let mut app = App::new(&tasks);
    ratatui::run(|term| app.run(term));
}
