use uuid::Uuid;

use crate::model::task::Task;

const TASKS_FILE: &str = "data/tasks.json";

pub struct TasksManager {
    tasks: Vec<Task>,
}

impl TasksManager {
    pub fn new() -> Self {
        TasksManager {
            tasks: Self::load_tasks(),
        }
    }

    pub fn load_tasks() -> Vec<Task> {
        let data = std::fs::read_to_string(TASKS_FILE).expect("Failed to read tasks file");
        serde_json::from_str(&data).expect("Failed to deserialize tasks")
    }

    pub fn save_tasks(&self) {
        let serialized =
            serde_json::to_string_pretty(&self.tasks).expect("Failed to serialize tasks");
        std::fs::write(TASKS_FILE, serialized).expect("Failed to write tasks to file");
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn iter_tree(&self, f: fn(&Task, depth: i32) -> bool) {
        for task in &self.tasks {
            task.iter_tree(0, f);
        }
    }

    pub fn flatten(&self) -> Vec<&Task> {
        let mut tasks = Vec::new();
        for task in &self.tasks {
            tasks.extend(task.flatten());
        }
        tasks
    }

    pub fn get_task(&mut self, ids: &[Uuid]) -> Option<&Task> {
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
}
