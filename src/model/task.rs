use std::{fmt::Debug, hash};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Goal {}

#[derive(Serialize, Deserialize, Clone)]
pub enum TaskTag {
    Other(String),
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,

    pub created_at: DateTime<Local>,
    pub ended_at: Option<DateTime<Local>>,

    pub sessions: Vec<Session>,

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
            sessions: Vec::new(),
            goals: Vec::new(),
            tags: Vec::new(),
            subtasks: Vec::new(),
            priority: 0,
        }
    }

    pub fn empty() -> Self {
        Self::new(String::new(), String::new())
    }

    pub fn add_subtask(&mut self, subtask: Task) {
        self.subtasks.push(subtask);
    }
    pub fn in_session(&self) -> bool {
        if let Some(session) = self.sessions.last() {
            session.end.is_none()
        } else {
            false
        }
    }

    fn start_session(&mut self) {
        if self.in_session() {
            return;
        }
        let session = Session {
            start: chrono::Local::now(),
            end: None,
        };
        self.sessions.push(session);
    }

    fn end_session(&mut self) {
        if let Some(session) = self.sessions.last_mut() {
            session.end = Some(chrono::Local::now());
        }
    }

    pub fn track(&mut self) -> bool {
        if self.in_session() {
            self.end_session();
            false
        } else {
            self.start_session();
            true
        }
    }

    pub fn elapsed_session_time(&self) -> chrono::Duration {
        if let Some(session) = self.sessions.last() {
            if let Some(end) = session.end {
                return end - session.start;
            } else {
                return chrono::Local::now() - session.start;
            }
        }

        chrono::Duration::zero()
    }

    pub fn toggle_completed(&mut self) {
        if self.is_completed() {
            self.ended_at = None;
        } else {
            self.ended_at = Some(chrono::Local::now());
        }
        self.end_session();
    }

    pub fn is_completed(&self) -> bool {
        self.ended_at.is_some()
    }

    pub fn to_string(&self) -> String {
        let mut out = format!("Task: {}", self.name);
        for subtask in &self.subtasks {
            out += &format!("\n\t{}", subtask.to_string());
        }
        out
    }

    pub fn iter_tree(&self, depth: i32, f: fn(&Task, depth: i32) -> bool) {
        if !f(self, depth) {
            return;
        }
        for subtask in &self.subtasks {
            subtask.iter_tree(depth + 1, f);
        }
    }

    pub fn flatten(&self) -> Vec<&Task> {
        let mut tasks = vec![self];
        for subtask in &self.subtasks {
            tasks.extend(subtask.flatten());
        }
        tasks
    }

    pub fn get_subtask(&self, ids: &[uuid::Uuid]) -> Option<&Task> {
        if ids.len() == 0 {
            return None;
        }
        if ids.last().expect("ids is empty") == &self.id {
            return Some(self);
        }

        for subtask in &self.subtasks {
            if let Some(task) = subtask.get_subtask(ids) {
                return Some(task);
            }
        }

        None
    }

    pub fn get_subtask_mut(&mut self, ids: &[uuid::Uuid]) -> Option<&mut Task> {
        if ids.len() == 0 {
            return None;
        }
        if ids.last().expect("ids is empty") == &self.id {
            return Some(self);
        }

        for subtask in &mut self.subtasks {
            if let Some(task) = subtask.get_subtask_mut(ids) {
                return Some(task);
            }
        }

        None
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
impl Eq for Task {}

impl hash::Hash for Task {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
