use std::{fmt::Debug, hash, path::PathBuf};

use chrono::{DateTime, Local};
use regex::Regex;
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
#[serde(default)]
#[derive(Default)]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,

    created_at: DateTime<Local>,
    ended_at: Option<DateTime<Local>>,

    sessions: Vec<Session>,

    goals: Vec<Goal>,
    tags: Vec<TaskTag>,
    pub subtasks: Vec<Task>,
    pub local_path: Option<PathBuf>,
    pub associations: Vec<String>,
}

impl Task {
    pub fn new(name: String, description: String) -> Self {
        Task {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            created_at: chrono::Local::now(),
            ..Default::default()
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

    pub fn start_session(&mut self) {
        if self.in_session() {
            return;
        }
        let session = Session {
            start: chrono::Local::now(),
            end: None,
        };
        self.sessions.push(session);
    }

    pub fn end_session(&mut self) {
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

    pub fn elapsed_daily_time(&self) -> chrono::Duration {
        let today = chrono::Local::now().date_naive();
        let mut total_duration = chrono::Duration::zero();

        for session in &self.sessions {
            let session_date = session.start.date_naive();
            if session_date == today {
                if let Some(end) = session.end {
                    total_duration = total_duration + (end - session.start);
                } else {
                    total_duration = total_duration + (chrono::Local::now() - session.start);
                }
            }
        }

        total_duration
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

    pub fn is_associated(&self, window: &str) -> bool {
        self.associations
            .iter()
            .any(|a| Regex::new(a).unwrap().is_match(window))
    }

    pub fn make_local_path(&mut self) {
        if let Some(path) = std::env::current_dir().ok() {
            self.local_path = Some(path);
        }
    }

    pub fn is_in_local_path(&self) -> bool {
        if let Ok(current_path) = std::env::current_dir()
            && let Some(local_path) = &self.local_path
        {
            return local_path == &current_path;
        }

        false
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

    pub fn iter_tree_mut<F: FnMut(&mut Task, i32) -> bool>(&mut self, depth: i32, f: &mut F) {
        if !f(self, depth) {
            return;
        }
        for subtask in &mut self.subtasks {
            subtask.iter_tree_mut(depth + 1, f);
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

    /// Calculates the total time spent on this task and all its subtasks 
    /// recursively within the given time range.
    /// overlapping parent/child sessions are both individually counted
    pub fn time_spent_recursive(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> chrono::Duration {
        let mut total_duration = self.time_spent(start, end);

        for subtask in &self.subtasks {
            total_duration = total_duration + subtask.time_spent_recursive(start, end);
        }

        total_duration
    }

    pub fn time_spent(&self, start: DateTime<Local>, end: DateTime<Local>) -> chrono::Duration {
        let mut total_duration = chrono::Duration::zero();

        for session in self.sessions.iter().rev() {
            let session_start = session.start;
            let session_end = session.end.unwrap_or_else(|| chrono::Local::now());

            if session_end < start {
                // sessions is sorted, so we can break early
                break;
            }

            if session_start > end {
                continue;
            }

            let overlap_start = session_start.max(start);
            let overlap_end = session_end.min(end);

            total_duration = total_duration + (overlap_end - overlap_start);
        }

        total_duration
    }

    pub fn total_time_spent(&self) -> chrono::Duration {
        let mut total_duration = chrono::Duration::zero();

        for session in &self.sessions {
            let session_end = session.end.unwrap_or_else(|| chrono::Local::now());
            total_duration = total_duration + (session_end - session.start);
        }

        total_duration
    }

    pub fn total_time_spent_recursive(&self) -> chrono::Duration {
        let mut total_duration = self.total_time_spent();

        for subtask in &self.subtasks {
            total_duration = total_duration + subtask.total_time_spent_recursive();
        }

        total_duration
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
