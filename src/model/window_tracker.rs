use chrono::{DateTime, Duration, Local};

use uuid::Uuid;

use crate::model::{
    tasks_manager,
    tracking_interface::{TrackingInterface, default_tracking_interface},
};

pub struct WindowTracker {
    tracked_tasks: Vec<(Uuid, DateTime<Local>)>,
    tracking_interface: Box<dyn TrackingInterface>,
    grace: Duration,

    last_pulse: DateTime<Local>, // TODO: persistent?
    heartbeat_interval: Duration,
}

impl WindowTracker {
    pub fn default() -> Self {
        Self::new(default_tracking_interface(), 60, 300)
    }

    pub fn new(interface: Box<dyn TrackingInterface>, grace: i64, heartbeat_interval: i64) -> Self {
        WindowTracker {
            tracked_tasks: Vec::new(),
            tracking_interface: interface,
            grace: Duration::seconds(grace),
            last_pulse: Local::now(),
            heartbeat_interval: Duration::seconds(heartbeat_interval),
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
        tasks_manager.iter_tree_mut(&mut |task, _| {
            let active_since = self.tracked_tasks.iter().find(|(id, _)| *id == task.id);
            if let Some(_) = active_since {
                task.end_session_at(self.last_pulse);
                task.start_session();
            }
            true
        });
    }

    pub fn track_tasks(&mut self, tasks_manager: &mut tasks_manager::TasksManager) {
        let now = Local::now();
        let active_window = self.get_active_window();
        let mut newly_tracked_tasks = Vec::new();

        if now - self.last_pulse >= self.heartbeat_interval {
            self.remove_downtime(tasks_manager);
        }
        self.last_pulse = now;

        tasks_manager.iter_tree_mut(&mut |task, _| {
            let is_active = active_window
                .as_ref()
                .is_some_and(|w| task.is_associated(&w));

            let active_since = self.tracked_tasks.iter().find(|(id, _)| *id == task.id);

            if let Some((_, last_activated_at)) = active_since {
                let grace_surpassed = now - *last_activated_at >= self.grace;
                let grace_active = task.elapsed_session_time() > self.grace;

                if is_active {
                    newly_tracked_tasks.push((task.id, now));
                    if !task.in_session() {
                        task.start_session();
                    }
                } else if grace_surpassed || !grace_active {
                    task.end_session();
                } else {
                    newly_tracked_tasks.push((task.id, *last_activated_at));
                }
            } else if is_active && !task.in_session() {
                newly_tracked_tasks.push((task.id, now));
                task.start_session();
            }

            true
        });

        self.tracked_tasks = newly_tracked_tasks;
    }
}
