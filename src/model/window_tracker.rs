use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::model::tasks_manager;

#[cfg(not(debug_assertions))]
const DEBOUNCE: Duration = Duration::from_secs(60);

#[cfg(debug_assertions)]
const DEBOUNCE: Duration = Duration::from_secs(5);

pub struct WindowTracker {
    activated_tasks: Vec<(Uuid, Instant)>,
}

impl WindowTracker {
    pub fn new() -> Self {
        WindowTracker {
            activated_tasks: Vec::new(),
        }
    }

    #[cfg(target_os = "linux")]
    fn kdotool_get_window_info(&self, window_id: &str) -> Option<String> {
        use std::process::Command;

        let window_title = Command::new("kdotool")
            .arg("getwindowname")
            .arg(window_id)
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let title = String::from_utf8_lossy(&output.stdout);
                    Some(title.to_string())
                } else {
                    None
                }
            });
        let window_class = Command::new("kdotool")
            .arg("getwindowclassname")
            .arg(window_id)
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let class = String::from_utf8_lossy(&output.stdout);
                    Some(class.to_string())
                } else {
                    None
                }
            });

        match (window_class, window_title) {
            (Some(class), Some(title)) => Some(format!("{}:{}", class.trim(), title.trim())),
            _ => None,
        }
    }

    #[cfg(target_os = "linux")]
    fn get_active_window_kde(&self) -> Option<String> {
        use std::process::Command;

        Command::new("kdotool")
            .arg("getactivewindow")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let window_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    self.kdotool_get_window_info(&window_id)
                } else {
                    None
                }
            })
    }

    #[cfg(target_os = "windows")]
    fn get_active_window_windows(&self) -> Option<String> {
        None
    }

    pub fn get_active_window(&self) -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            match std::env::var("XDG_CURRENT_DESKTOP") {
                Ok(desktop) if desktop.to_lowercase().contains("kde") => {
                    self.get_active_window_kde()
                }
                _ => None,
            }
        }
        #[cfg(target_os = "windows")]
        {
            self.get_active_window_windows()
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
