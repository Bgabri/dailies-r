use uuid::Uuid;

use crate::model::tasks_manager;

pub struct WindowTracker {
    active_window: Option<String>,
    activated_tasks: Vec<Uuid>,
}

impl WindowTracker {
    pub fn new() -> Self {
        WindowTracker {
            active_window: None,
            activated_tasks: Vec::new(),
        }
    }
    #[cfg(target_os = "linux")]
    pub fn find_active_window(&self) -> Option<String> {
        let window_info = kdotool::get_active_window_info();
        window_info
            .ok()
            .map(|info| format!("{}:{}", info.class_name, info.title))
    }
    #[cfg(not(target_os = "linux"))]
    fn find_active_window(&self) -> Option<String> {
        None
    }

    fn update_active_window(&mut self) -> bool {
        let prev_window = self.active_window.clone();
        self.active_window = self.find_active_window();

        prev_window != self.active_window
    }

    pub fn track_tasks(&mut self, tasks_manager: &mut tasks_manager::TasksManager) {
        if !self.update_active_window() {
            return;
        }

        tasks_manager.iter_tree_mut(&mut |task, _depth| {
            if self.activated_tasks.contains(&task.id) {
                task.end_session();
            }
            true
        });

        let mut tracked_tasks = Vec::new();

        if let Some(active_window) = &self.active_window {
            tasks_manager.iter_tree_mut(&mut |task, _depth| {
                if task.is_associated(active_window) && !task.in_session() {
                    tracked_tasks.push(task.id);
                    task.start_session();
                }
                true
            });
        }

        self.activated_tasks = tracked_tasks;
    }
}
