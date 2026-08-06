use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::model::tasks_manager;

const DEBOUNCE: Duration = Duration::from_secs(60);

pub struct WindowTracker {
    activated_tasks: Vec<(Uuid, Instant)>,
}

impl WindowTracker {
    pub fn new() -> Self {
        WindowTracker {
            activated_tasks: Vec::new(),
        }
    }
    pub fn get_active_window(&self) -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            let window_info = kdotool::get_active_window_info();
            window_info
                .ok()
                .map(|info| format!("{}:{}", info.class_name, info.title))
        }
        #[cfg(not(target_os = "linux"))]
        {
            None
        }
    }

    fn get_open_windows(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn track_tasks(&mut self, tasks_manager: &mut tasks_manager::TasksManager) {
        let now = Instant::now();
        let active_window = self.get_active_window();
        let mut tracked_tasks = Vec::new();

        tasks_manager.iter_tree_mut(&mut |task, _| {
            if let Some((_, last_activated_at)) =
                self.activated_tasks.iter().find(|(id, _)| *id == task.id)
            {
                if let Some(active_window) = &active_window
                    && task.is_associated(active_window)
                {
                    tracked_tasks.push((task.id, now));
                    if !task.in_session() {
                        task.start_session();
                    }
                } else if now.duration_since(*last_activated_at) >= DEBOUNCE {
                    task.end_session();
                } else {
                    tracked_tasks.push((task.id, *last_activated_at));
                }
            } else if let Some(active_window) = &active_window
                && task.is_associated(active_window)
                && !task.in_session()
            {
                tracked_tasks.push((task.id, now));
                task.start_session();
            }

            true
        });

        self.activated_tasks = tracked_tasks;
    }
}
