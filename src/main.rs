mod model;
use std::io;

mod view;
use view::dailies_app::DailiesApp;

use crate::model::tasks_manager::TasksManager;

fn main() -> io::Result<()> {
    let tasks_manager = TasksManager::new();
    let mut app = DailiesApp::new(tasks_manager);
    ratatui::run(|term| app.run(term))
    // Ok(())
}
