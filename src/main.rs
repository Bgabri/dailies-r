use std::io;
use view::dailies_app::DailiesApp;

mod model;
mod view;

use crate::model::tasks_manager::TasksManager;

use crate::view::dailies_cli::DailiesArgs;

fn main() -> io::Result<()> {
    let args = DailiesArgs::get_args();
    let tasks_manager = TasksManager::new();

    match args.command {
        Some(view::dailies_cli::Subcommands::Interactive) | None => {
            let mut app = DailiesApp::new(tasks_manager);
            ratatui::run(|term| app.run(term))
        }
    }
}
