use std::io;
use view::dailies_app::DailiesApp;

mod model;
mod view;

use crate::model::tasks_manager::TasksManager;

use crate::model::window_tracker::WindowTracker;
use crate::view::dailies_cli::{DailiesArgs, Subcommands};

fn main() -> io::Result<()> {
    let args = DailiesArgs::get_args();
    let tasks_manager = TasksManager::new();

    match args.command {
        Some(Subcommands::Interactive) | None => {
            let mut app = DailiesApp::new(tasks_manager);
            ratatui::run(|term| app.run(term))
        }
        Some(Subcommands::Windows) => {
            let open_windows = WindowTracker::get_open_windows();
            for window in open_windows {
                println!("'{}'", window);
            }
            Ok(())
        }
    }
}
