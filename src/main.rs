use std::fs::File;
use std::io::{self, BufRead, BufReader};

use chrono::{DateTime, Local, Utc};
use uuid::Uuid;

enum Tag {
    Fun,
    Dev,
    Uni,
    Work,

    Other(String),
}

struct Task {
    name: String,
    description: String,
    subtasks: Vec<Task>,
    sessions: Vec<DateTime<Local>>,
    in_session: bool,
    uuid: Uuid,
    priority: u8,
    tags: Vec<Tag>,
    created_at: DateTime<Local>,
    ended_at: Option<DateTime<Local>>,
    completed: bool,
}

impl Task {
    fn new() -> Self {
        Task {
            name: String::new(),
            description: String::new(),
            subtasks: Vec::new(),
            sessions: Vec::new(),
            in_session: false,
            uuid: Uuid::nil(),
            priority: 0,
            tags: Vec::new(),
            created_at: Local::now(),
            ended_at: None,
            completed: false,
        }
    }

    fn add_subtask(&mut self, subtask: Task) {
        self.subtasks.push(subtask);
    }

}

fn main() {
    let t0 = Task::new();

    let mut t1 = Task::new();
    t1.add_subtask(t0);

    dbg!("{t}");
    dbg!("{t0}");
}
