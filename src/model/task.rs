use std::{fmt::Debug, hash};

use chrono::{DateTime, Local};

pub struct Session {
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

pub struct Goal {}

pub enum TaskTag {
    Other(String),
}

pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,

    pub created_at: DateTime<Local>,
    pub ended_at: Option<DateTime<Local>>,
    pub completed: Option<bool>,

    pub sessions: Vec<Session>,
    pub in_session: bool,

    pub goals: Vec<Goal>,
    pub tags: Vec<TaskTag>,
    pub subtasks: Vec<Task>,
    pub priority: u8,
}

impl Task {
    pub fn new(name: String, description: String) -> Self {
        Task {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            created_at: chrono::Local::now(),
            ended_at: None,
            completed: None,
            sessions: Vec::new(),
            in_session: false,
            goals: Vec::new(),
            tags: Vec::new(),
            subtasks: Vec::new(),
            priority: 0,
        }
    }

    pub fn add_subtask(&mut self, subtask: Task) {
        self.subtasks.push(subtask);
    }

    pub fn to_string(&self) -> String {
        let mut out = format!("Task: {}", self.name);
        for subtask in &self.subtasks {
            out += &format!("\n\t{}", subtask.to_string());
        }
        out
    }
}


impl Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task{{{}}}", self.name)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    
}
impl Eq for Task {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
}

impl hash::Hash for Task {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id;
    }
}