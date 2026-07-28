mod model;
use std::io;

mod view;
use view::dailies_app::DailiesApp;

use crate::model::tasks_manager::TasksManager;

fn main() -> io::Result<()> {
    let task_manager = TasksManager::new();
    let mut app = DailiesApp::new(task_manager);

    ratatui::run(|term| app.run(term))
    // Ok(())
}
