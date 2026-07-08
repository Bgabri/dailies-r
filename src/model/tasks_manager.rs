use uuid::Uuid;

use crate::model::task::Task;

pub struct TasksManager {
    tasks: Vec<Task>,
}

impl TasksManager {
    pub fn new() -> Self {
        TasksManager {
            tasks: Self::test_tasks(),
        }
    }

    fn test_tasks() -> Vec<Task> {
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
        vec![task]
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
