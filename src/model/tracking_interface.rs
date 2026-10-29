use std::process::Command;

pub trait TrackingInterface {
    fn get_active_window(&self) -> Option<String>;
    fn get_open_windows(&self) -> Vec<String>;
}

pub fn default_tracking_interface() -> Box<dyn TrackingInterface> {
    #[cfg(target_os = "linux")]
    {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_default()
            .to_lowercase();
        if desktop.contains("kde") {
            Box::new(KdeTracking)
        } else {
            Box::new(UnimplementedTracking)
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        Box::new(UnimplementedTracking)
    }
}

pub struct UnimplementedTracking;

// TODO: throw errors
impl TrackingInterface for UnimplementedTracking {
    fn get_active_window(&self) -> Option<String> {
        None
    }

    fn get_open_windows(&self) -> Vec<String> {
        Vec::new()
    }
}

pub struct KdeTracking;

impl KdeTracking {
    fn kdotool_get_window_info(window_id: &str) -> Option<String> {
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
}

impl TrackingInterface for KdeTracking {
    fn get_active_window(&self) -> Option<String> {
        Command::new("kdotool")
            .arg("getactivewindow")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let window_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    Self::kdotool_get_window_info(&window_id)
                } else {
                    None
                }
            })
    }

    fn get_open_windows(&self) -> Vec<String> {
        let windows = Command::new("kdotool")
            .arg("search")
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter_map(|window_id| Self::kdotool_get_window_info(window_id.trim()))
                    .collect()
            })
            .unwrap_or_default();

        windows
    }
}
