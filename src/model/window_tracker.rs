use chrono::{DateTime, Duration, Local};

use uuid::Uuid;

use crate::model::{
    tasks_manager,
    tracking_interface::{TrackingInterface, default_tracking_interface},
};

pub struct WindowTracker {
    tracked_tasks: Vec<(Vec<Uuid>, DateTime<Local>)>,
    tracking_interface: Box<dyn TrackingInterface>,
    grace: Duration,

    last_pulse: DateTime<Local>, // TODO: persistent?
    heartbeat_interval: Duration,
}

impl WindowTracker {
    pub fn default() -> Self {
        Self::new(default_tracking_interface(), 60, 5)
    }

    pub fn new(interface: Box<dyn TrackingInterface>, grace: i64, heartbeat_interval: i64) -> Self {
        WindowTracker {
            tracked_tasks: Vec::new(),
            tracking_interface: interface,
            grace: Duration::seconds(grace),
            last_pulse: Local::now(),
            heartbeat_interval: Duration::minutes(heartbeat_interval),
        }
    }

    pub fn with_grace(mut self, grace: i64) -> Self {
        self.grace = Duration::seconds(grace);
        self
    }
    pub fn with_heartbeat_interval(mut self, heartbeat_interval: i64) -> Self {
        self.heartbeat_interval = Duration::minutes(heartbeat_interval);
        self
    }

    pub fn get_active_window(&self) -> Option<String> {
        self.tracking_interface.get_active_window()
    }

    pub fn get_open_windows(&self) -> Vec<String> {
        self.tracking_interface.get_open_windows()
    }

    fn remove_downtime(&mut self, tasks_manager: &mut tasks_manager::TasksManager) {
        for (ids, _) in self.tracked_tasks.iter_mut() {
            if let Some(task) = tasks_manager.get_task_mut(&ids) {
                task.end_session_at(self.last_pulse);
                task.start_session();
            };
        }
    }

    fn check_tracked_tasks(&mut self, tasks_manager: &mut tasks_manager::TasksManager) {
        let now = Local::now();
        let active_window = self.get_active_window();

        self.tracked_tasks.retain_mut(|(ids, last_active_at)| {
            let Some(task) = tasks_manager.get_task_mut(&ids) else {
                return false;
            };
            let is_active = active_window
                .as_deref()
                .is_some_and(|w| task.is_associated(w));

            let grace_surpassed = now - *last_active_at >= self.grace;
            let grace_active = task.elapsed_session_time() > self.grace;

            if is_active {
                *last_active_at = now;
            } else if grace_surpassed || !grace_active {
                task.end_session();
                return false;
            }

            true
        });
    }

    fn track_tasks(&mut self, tasks_manager: &mut tasks_manager::TasksManager) {
        let Some(active_window) = self.get_active_window() else {
            return;
        };

        tasks_manager.iter_tree_mut(&mut |task, ids| {
            let is_active = task.is_associated(&active_window);
            if is_active && !task.in_session() {
                self.tracked_tasks.push((ids, Local::now()));
                task.start_session();
            }

            true
        });
    }

    pub fn auto_track(&mut self, tasks_manager: &mut tasks_manager::TasksManager) {
        let now = Local::now();

        if now - self.last_pulse >= self.heartbeat_interval {
            self.remove_downtime(tasks_manager);
        }
        self.last_pulse = now;

        self.check_tracked_tasks(tasks_manager);
        self.track_tasks(tasks_manager);
    }
}
