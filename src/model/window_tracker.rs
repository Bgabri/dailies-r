use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::model::{tasks_manager, tracking_interface::TrackingInterface};

#[cfg(not(debug_assertions))]
const DEBOUNCE: Duration = Duration::from_secs(60);

#[cfg(debug_assertions)]
const DEBOUNCE: Duration = Duration::from_secs(5);

pub struct WindowTracker {
    activated_tasks: Vec<(Uuid, Instant)>,
    tracking_interface: Box<dyn TrackingInterface>,
}

impl WindowTracker {
    pub fn new(interface: Box<dyn TrackingInterface>) -> Self {
        WindowTracker {
            activated_tasks: Vec::new(),
            tracking_interface: interface,
        }
    }

    pub fn get_active_window(&self) -> Option<String> {
        self.tracking_interface.get_active_window()
    }

    pub fn get_open_windows(&self) -> Vec<String> {
        self.tracking_interface.get_open_windows()
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
                } else if now.duration_since(*last_activated_at) >= DEBOUNCE
                    || task
                        .elapsed_session_time()
                        .to_std()
                        .map_or(false, |d| d <= DEBOUNCE)
                {
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
