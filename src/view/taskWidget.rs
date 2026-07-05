use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::model::task::Task;

pub struct TaskWidget<'a> {
    task: &'a Task,
}

impl<'a> TaskWidget<'a> {
    pub fn new(task: &'a Task) -> Self {
        TaskWidget { task }
    }
}

impl<'a> Widget for TaskWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let name = Span::styled(self.task.name.clone(), Modifier::BOLD);
        let description = Span::raw(&self.task.description);
        let line = Line::from(vec![name, description]);
        line.render(area, buf);
    }
}

pub struct TasksList<'a> {
    state: TreeState<&'a Task>,
    items: Vec<TreeItem<'a, &'a Task>>,
}

impl<'a> TasksList<'a> {
    fn tasks_tree(task: &Task) -> Result<TreeItem<'a, &Task>, std::io::Error> {
        if task.subtasks.len() == 0 {
            return Ok(TreeItem::new_leaf(&task, task.name.clone()));
        }

        let mut items = Vec::new();
        for sub_task in &task.subtasks {
            let item = Self::tasks_tree(sub_task)?;
            items.push(item);
        }
        return TreeItem::new(&task, task.name.clone(), items);
    }

    pub fn new(tasks: &'a Vec<Task>) -> Self {
        let mut items: Vec<TreeItem<'a, &'a Task>> = Vec::new();
        for task in tasks {
            let item = Self::tasks_tree(task).expect("duplicate tasks found");
            items.push(item);
        }

        Self {
            state: TreeState::default(),
            items,
        }
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
        // if self.state.selected().len() > 0 {
        //     self.dbg_hud = format!(
        //         "{:?}",
        //         self.state.selected().last().expect("no tree item selected")
        //     );
        // }
        match key_event.code {
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

impl Widget for &mut TasksList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        StatefulWidget::render(tree, area, buf, &mut self.state)
    }
}

pub struct TaskApp<'a> {
    title: String,
    tasks: &'a Vec<Task>,
    exit: bool,
    tasks_list: TasksList<'a>,
}

impl<'a> TaskApp<'a> {
    pub fn new(tasks: &'a Vec<Task>) -> Self {
        TaskApp {
            title: "Task Manager".to_string(),
            tasks,
            tasks_list: TasksList::new(tasks),
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if (key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc) {
                    self.exit();
                }
                self.tasks_list.handle_key_event(key_event)
            }
            Event::Mouse(mouse_event) => self.tasks_list.handle_mouse_event(mouse_event),
            _ => {}
        }
        Ok(())
    }
}

impl<'a> Widget for &mut TaskApp<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let window = Block::bordered()
            .title(Line::from(self.title.clone()).centered())
            .border_set(border::ROUNDED);

        self.tasks_list.render(window.inner(area), buf);
        window.render(area, buf);
    }
}
