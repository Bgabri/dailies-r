mod model;
use std::io;

mod view;
use view::task_widget::TaskApp;

use crate::model::tasks_manager::TasksManager;

fn main() -> io::Result<()> {
    let mut task_manager = TasksManager::new();
    let mut app = TaskApp::new(task_manager);

    ratatui::run(|term| app.run(term))
    // Ok(())
}
