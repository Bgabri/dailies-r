use chrono::{DateTime, Datelike, Duration, Local, NaiveTime};
use uuid::Uuid;

use crate::model::task::Task;
use std::path::PathBuf;

const NAIVE_TIME_MAX: NaiveTime = NaiveTime::from_hms_opt(23, 59, 59).unwrap();

pub struct TasksManager {
    tasks: Vec<Task>,
}

impl TasksManager {
    pub fn new() -> Self {
        TasksManager {
            tasks: Self::load_tasks(),
        }
    }

    #[cfg(debug_assertions)]
    fn tasks_file_path() -> PathBuf {
        PathBuf::from("data/tasks.json")
    }

    #[cfg(not(debug_assertions))]
    fn tasks_file_path() -> PathBuf {
        let mut dir = dirs::data_dir().expect("Could not find data directory");
        dir.push("dailies-r");
        dir.push("tasks.json");
        dir
    }

    pub fn load_tasks() -> Vec<Task> {
        let path = Self::tasks_file_path();
        if !path.exists() {
            return Vec::new();
        }
        let data = std::fs::read_to_string(&path).expect("Failed to read tasks file");
        serde_json::from_str(&data).expect("Failed to deserialize tasks")
    }

    pub fn save_tasks(&self) {
        let path = Self::tasks_file_path();
        if let Some(parent_dir) = path.parent() {
            std::fs::create_dir_all(parent_dir).expect("Failed to create tasks directory");
        }

        let serialized =
            serde_json::to_string_pretty(&self.tasks).expect("Failed to serialize tasks");
        std::fs::write(&path, serialized).expect("Failed to write tasks to file");
    }

    pub fn add_task(&mut self, parent_ids: &[Uuid], task: Task) {
        if let Some(parent) = self.get_task_mut(parent_ids) {
            parent.subtasks.push(task);
            return;
        } else if parent_ids.is_empty() {
            self.tasks.push(task);
            return;
        }
    }

    pub fn update_task(&mut self, ids: &[Uuid], updated_task: Task) {
        if let Some(task) = self.get_task_mut(ids) {
            *task = updated_task;
            return;
        }
    }

    pub fn delete_task(&mut self, ids: &[Uuid]) {
        if let Some((id, parent_ids)) = ids.split_last() {
            if let Some(parent) = self.get_task_mut(parent_ids) {
                parent.subtasks.retain(|t| &t.id != id);
            } else if parent_ids.is_empty() {
                self.tasks.retain(|t| &t.id != id);
            }
        }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn iter_tree<F: FnMut(&Task, Vec<Uuid>) -> bool>(&self, f: &mut F) {
        for task in &self.tasks {
            task.iter_tree(vec![task.id], f);
        }
    }

    pub fn iter_tree_mut<F: FnMut(&mut Task, Vec<Uuid>) -> bool>(&mut self, f: &mut F) {
        for task in &mut self.tasks {
            task.iter_tree_mut(vec![task.id], f);
        }
    }

    pub fn flatten(&self) -> Vec<&Task> {
        let mut tasks = Vec::new();
        for task in &self.tasks {
            tasks.extend(task.flatten());
        }
        tasks
    }

    pub fn get_task(&self, ids: &[Uuid]) -> Option<&Task> {
        for task in &self.tasks {
            if let Some(t) = task.get_subtask(ids) {
                return Some(t);
            }
        }
        None
    }

    pub fn get_task_mut(&mut self, ids: &[Uuid]) -> Option<&mut Task> {
        for task in &mut self.tasks {
            if let Some(t) = task.get_subtask_mut(ids) {
                return Some(t);
            }
        }
        None
    }

    pub fn get_daily_report(&self, day: DateTime<Local>) -> Vec<(String, Duration)> {
        let day_start = day.with_time(NaiveTime::MIN).unwrap();
        let day_end = day.with_time(NAIVE_TIME_MAX).unwrap();

        let mut report: Vec<(String, Duration)> = Vec::new();

        for task in self.get_tasks() {
            let time_spent = task.time_spent_recursive(day_start, day_end);
            report.push((task.name.clone(), time_spent));
        }

        report
    }

    pub fn get_weekly_report(&self, day: DateTime<Local>) -> Vec<(String, Vec<Duration>)> {
        let mut report: Vec<(String, Vec<Duration>)> = Vec::new();

        let today_offset = day.weekday().num_days_from_monday() as i64;
        let week_start = day.with_time(NaiveTime::MIN).unwrap() - Duration::days(today_offset);
        for task in self.get_tasks() {
            let mut time_spent_per_day: Vec<Duration> = Vec::new();
            for day_offset in 0..7 {
                let previous_week_offset = if day_offset > today_offset { 7 } else { 0 };
                let day_start = week_start + Duration::days(day_offset - previous_week_offset);
                let day_end = day_start + Duration::days(1);
                let time_spent = task.time_spent_recursive(day_start, day_end);
                time_spent_per_day.push(time_spent);
            }
            report.push((task.name.clone(), time_spent_per_day));
        }

        report
    }
}
