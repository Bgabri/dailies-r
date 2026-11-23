use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::model::{
    tasks_manager,
    tracking_interface::{TrackingInterface, default_tracking_interface},
};


pub struct WindowTracker {
    activated_tasks: Vec<(Uuid, Instant)>,
    tracking_interface: Box<dyn TrackingInterface>,
    debounce: Duration,
}

impl WindowTracker {
    pub fn default() -> Self {
        WindowTracker {
            activated_tasks: Vec::new(),
            tracking_interface: default_tracking_interface(),
            debounce: Duration::from_secs(60),
        }
    }

    pub fn new(interface: Box<dyn TrackingInterface>, debounce: u64) -> Self {
        WindowTracker {
            activated_tasks: Vec::new(),
            tracking_interface: interface,
            debounce: Duration::from_secs(debounce),
        }
    }
    pub fn with_debounce(mut self, debounce: u64) -> Self {
        self.debounce = Duration::from_secs(debounce);
        self
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
                } else if now.duration_since(*last_activated_at) >= self.debounce
                    || task
                        .elapsed_session_time()
                        .to_std()
                        .map_or(false, |d| d <= self.debounce)
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
