use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};
use uuid::Uuid;

use crate::model::{task::Task, tasks_manager::TasksManager};

pub struct TasksList<'a> {
    tasks_manager: &'a mut TasksManager,
    state: TreeState<Uuid>,
}

impl<'a> TasksList<'a> {
    pub fn new(tasks_manager: &'a mut TasksManager) -> Self {
        Self {
            state: TreeState::default(),
            tasks_manager,
        }
    }

    fn tasks_line(&self, task: &Task) -> Line<'a> {
        if (task.in_session()) {
            return Line::from(vec![
                task.name.clone().magenta().bold().underlined(),
                ": ".into(),
                task.description.clone().italic().dim(),
                task.elapsed_session_time()
                    .to_string()
                    .green()
                    .bold()
                    .underlined(),
            ]);
        }
        Line::from(vec![
            task.name.clone().bold(),
            ": ".into(),
            task.description.clone().italic().dim(),
        ])
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
            KeyCode::Char('p' | ' ') => self.track_selected(),
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

pub struct TaskApp<'a> {
    title: String,
    // task_manager: &'a mut TasksManager,
    exit: bool,
    tasks_list: TasksList<'a>,
}

impl<'a> TaskApp<'a> {
    pub fn new(task_manager: &'a mut TasksManager) -> Self {
        TaskApp {
            title: "Task Manager".to_string(),
            // task_manager,
            tasks_list: TasksList::new(task_manager),
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
        let frame_timeout = Duration::from_secs_f64(1.0/4.0);
        if !event::poll(frame_timeout)? {
            return Ok(());
        }

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
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
