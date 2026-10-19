
use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Clear, StatefulWidget, Widget},
};
use uuid::Uuid;

use crate::model::{task::Task, tasks_manager::TasksManager};
use crate::view::{
    confirm_view::{ConfirmView, ConfirmViewState},
    dailies_app::DailiesAppEvent,
    edit_task_view::EditTaskViewState,
    tasks_tree::{TasksTreeView, TasksTreeViewState},
};

pub enum ManagerViewWindow {
    ListView,
    EditTaskView(Vec<Uuid>, EditTaskViewState),
    AddTaskView(Vec<Uuid>, EditTaskViewState),
    DeleteTaskView(Vec<Uuid>, ConfirmViewState<'static>),
}

pub enum ManagerViewEvent {
    Exit,
    EditTask(Vec<Uuid>),
    AddTask(Vec<Uuid>),
    TrackTask(Vec<Uuid>),
    DeleteTask(Vec<Uuid>),
    CompleteTask(Vec<Uuid>),
    SwitchTab,
    Save,
    Cancel,
    None,
}

pub struct ManagerViewState {
    tasks_tree_state: TasksTreeViewState,
    window: ManagerViewWindow,
}

impl ManagerViewState {
    pub fn new() -> Self {
        ManagerViewState {
            tasks_tree_state: TasksTreeViewState::new(),
            window: ManagerViewWindow::ListView,
        }
    }

    pub fn handle_events(
        &mut self,
        event: Event,
        tasks_manager: &mut TasksManager,
    ) -> DailiesAppEvent {
        let manager_event: ManagerViewEvent = match &mut self.window {
            ManagerViewWindow::ListView => self.tasks_tree_state.handle_events(event),
            ManagerViewWindow::EditTaskView(_, task_editor_state) => {
                task_editor_state.handle_events(event)
            }
            ManagerViewWindow::AddTaskView(_, task_editor_state) => {
                task_editor_state.handle_events(event)
            }
            ManagerViewWindow::DeleteTaskView(_, confirm_view_state) => {
                confirm_view_state.handle_events(event)
            }
        };

        self.transition(manager_event, tasks_manager)
    }

    fn transition(
        &mut self,
        event: ManagerViewEvent,
        tasks_manager: &mut TasksManager,
    ) -> DailiesAppEvent {
        match event {
            ManagerViewEvent::Exit => DailiesAppEvent::Exit,
            ManagerViewEvent::EditTask(task_ids) => {
                if let Some(task) = tasks_manager.get_task(&task_ids) {
                    self.window = ManagerViewWindow::EditTaskView(
                        task_ids,
                        EditTaskViewState::new(task.clone()),
                    );
                    DailiesAppEvent::FreezeTab
                } else {
                    DailiesAppEvent::None
                }
            }
            ManagerViewEvent::AddTask(parent_ids) => {
                self.window = ManagerViewWindow::AddTaskView(
                    parent_ids,
                    EditTaskViewState::new(Task::empty()),
                );
                DailiesAppEvent::FreezeTab
            }

            ManagerViewEvent::TrackTask(task_ids) => {
                tasks_manager
                    .get_task_mut(&task_ids)
                    .and_then(|t| Some(t.track()));
                DailiesAppEvent::FreezeTab
            }
            ManagerViewEvent::CompleteTask(task_ids) => {
                tasks_manager
                    .get_task_mut(&task_ids)
                    .and_then(|t| Some(t.toggle_completed()));
                DailiesAppEvent::FreezeTab
            }
            ManagerViewEvent::DeleteTask(task_ids) => {
                if let Some(task) = tasks_manager.get_task(&task_ids) {
                    let message: Line = Line::from(vec![
                        "Delete task '".into(),
                        task.name.clone().italic(),
                        "'?".into(),
                    ]);
                    self.window =
                        ManagerViewWindow::DeleteTaskView(task_ids, ConfirmViewState::new(message));
                    DailiesAppEvent::FreezeTab
                }
                else {
                    DailiesAppEvent::None
                }
            }

            ManagerViewEvent::Save => {
                match &mut self.window {
                    ManagerViewWindow::EditTaskView(task_ids, task_editor_state) => {
                        let task = task_editor_state.get_task();
                        tasks_manager.update_task(task_ids, task);
                    }
                    ManagerViewWindow::AddTaskView(parent_ids, task_editor_state) => {
                        let task = task_editor_state.get_task();
                        tasks_manager.add_task(parent_ids, task);
                    }
                    ManagerViewWindow::DeleteTaskView(task_ids, _) => {
                        tasks_manager.delete_task(task_ids);
                    }
                    _ => {}
                }
                self.window = ManagerViewWindow::ListView;
                DailiesAppEvent::UnFreezeTab
            }

            ManagerViewEvent::Cancel => {
                self.window = ManagerViewWindow::ListView;
                DailiesAppEvent::UnFreezeTab
            }
            ManagerViewEvent::None | _ => {
                DailiesAppEvent::None
            }
        }
    }
}

pub struct ManagerView<'a> {
    tasks_manager: &'a TasksManager,
}

impl<'a> ManagerView<'a> {
    pub fn new(tasks_manager: &'a TasksManager) -> Self {
        ManagerView {
            tasks_manager: tasks_manager,
        }
    }

    fn render_edit_popup(
        task_editor_state: &mut EditTaskViewState,
        title: &str,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let mut task_editor = crate::view::edit_task_view::EditTaskView::new();
        let popup_block = Block::bordered()
            .title(Line::from(vec![
                title.into(),
                ": ".into(),
                task_editor_state.get_task_id().to_string().dim().into(),
            ]))
            .border_set(border::ROUNDED);
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Length(10));

        Clear.render(centered_area, buf);
        task_editor.render(popup_block.inner(centered_area), buf, task_editor_state);
        popup_block.render(centered_area, buf);
    }
}

impl<'a> StatefulWidget for &mut ManagerView<'a> {
    type State = ManagerViewState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let tasks_list = TasksTreeView::new(self.tasks_manager.get_tasks());
        tasks_list.render(area, buf, &mut state.tasks_tree_state);

        match &mut state.window {
            ManagerViewWindow::AddTaskView(_, task_editor_state) => {
                ManagerView::render_edit_popup(task_editor_state, "Add Task", area, buf);
            }
            ManagerViewWindow::EditTaskView(_, task_editor_state) => {
                ManagerView::render_edit_popup(task_editor_state, "Edit Task", area, buf);
            }
            ManagerViewWindow::DeleteTaskView(_, confirm_view_state) => {
                let centered_area =
                    area.centered(Constraint::Percentage(60), Constraint::Length(3));

                Clear.render(centered_area, buf);
                let confirm_view = ConfirmView::new();

                // TODO: Fix the lifetime issue here
                confirm_view.render(centered_area, buf, &mut confirm_view_state.clone());
            }
            _ => {}
        }
    }
}
