mod model;
use std::io;

use model::task::Task;

mod view;
use view::taskWidget::{TaskApp, TaskWidget};

use crate::view::taskWidget::TasksList;

fn main()-> io::Result<()> {
    let mut task = Task::new("My Task".to_string(), "This is a sample task.".to_string());
    let mut subtask1 = Task::new(
        "Subtask 1".to_string(),
        "This is the first subtask.".to_string(),
    );
    let subtask2 = Task::new(
        "Subtask 2".to_string(),
        "This is the second subtask.".to_string(),
    );
    let subsubtask = Task::new(
        "Sub-subtask".to_string(),
        "This is a sub-subtask.".to_string(),
    );
    subtask1.add_subtask(subsubtask);
    task.add_subtask(subtask1);
    task.add_subtask(subtask2);
    let tasks = vec![task];
    let mut app = TaskApp::new(&tasks);

    ratatui::run(|term| app.run(term))
}
