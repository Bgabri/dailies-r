use std::{io, time::Duration};

use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Offset, Rect},
    symbols::border,
    widgets::{Block, Clear, StatefulWidget, Tabs, Widget},
};

use crate::{
    model::tasks_manager::TasksManager,
    view::{
        report_view::{ReportView, ReportViewState},
        task_manager_view::{ManagerView, ManagerViewState},
    },
};

pub enum DailiesAppEvent {
    Exit,
    FreezeTab,
    UnFreezeTab,
    None,
}

#[derive(PartialEq)]
pub enum DailiesAppWindow {
    ManagerTab,
    ReportTab,
}
pub struct DailiesApp {
    tasks_manager: TasksManager,
    window: DailiesAppWindow,

    manager_view_state: ManagerViewState,
    report_view_state: ReportViewState,

    frozen: bool,
    exit: bool,
}

impl DailiesApp {
    pub fn new(tasks_manager: TasksManager) -> Self {
        DailiesApp {
            window: DailiesAppWindow::ManagerTab,
            manager_view_state: ManagerViewState::new(),
            report_view_state: ReportViewState::new(),
            tasks_manager,
            exit: false,
            frozen: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.handle_events()?;
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn next_tab(&mut self) {
        if self.frozen {
            return;
        }
        self.window = match self.window {
            DailiesAppWindow::ManagerTab => DailiesAppWindow::ReportTab,
            DailiesAppWindow::ReportTab => DailiesAppWindow::ManagerTab,
        };
    }

    fn previous_tab(&mut self) {
        if self.frozen {
            return;
        }
        self.window = match self.window {
            DailiesAppWindow::ManagerTab => DailiesAppWindow::ReportTab,
            DailiesAppWindow::ReportTab => DailiesAppWindow::ManagerTab,
        };
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let frame_timeout = Duration::from_secs_f64(1.0);
        if !event::poll(frame_timeout)? {
            return Ok(());
        }

        let event = event::read()?;
        if let Event::Key(KeyEvent {
            code: event::KeyCode::Char('c' | 'd' | 'q'),
            modifiers: event::KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.exit();
            return Ok(());
        }

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                ..
            }) => {
                self.previous_tab();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab, ..
            }) => {
                self.next_tab();
            }
            _ => {}
        }

        let app_event = match self.window {
            DailiesAppWindow::ManagerTab => self
                .manager_view_state
                .handle_events(event, &mut self.tasks_manager),
            DailiesAppWindow::ReportTab => DailiesAppEvent::None,
        };

        self.tasks_manager.save_tasks();

        self.transition(app_event);

        Ok(())
    }

    fn transition(&mut self, event: DailiesAppEvent) {
        match event {
            DailiesAppEvent::Exit => self.exit(),
            DailiesAppEvent::FreezeTab => self.frozen = true,
            DailiesAppEvent::UnFreezeTab => self.frozen = false,
            DailiesAppEvent::None => {}
        }
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let tabs = Tabs::new(vec!["Task Manager", "Report"]).select(match self.window {
            DailiesAppWindow::ManagerTab => 0,
            DailiesAppWindow::ReportTab => 1,
        });
        tabs.render(area + Offset::new(1, 0), buf);
    }
}

impl Widget for &mut DailiesApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let window = Block::bordered().border_set(border::ROUNDED);
        match self.window {
            DailiesAppWindow::ManagerTab => {
                let mut manager_view = ManagerView::new(&self.tasks_manager);
                manager_view.render(window.inner(area), buf, &mut self.manager_view_state);
            }
            DailiesAppWindow::ReportTab => {
                let report_view =
                    ReportView::new(self.tasks_manager.get_weekly_report(Local::now()));
                report_view.render(window.inner(area), buf, &mut self.report_view_state);
            }
        }

        window.render(area, buf);
        self.render_tabs(area, buf);
    }
}
