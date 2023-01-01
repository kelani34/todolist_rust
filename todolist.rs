use std::io;

struct Task {
    name: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn mark_task_complete(&mut self, task_name: &str) {
        for task in self.tasks.iter_mut() {
            if task.name == task_name {
                task.completed = true;
                break;
            }
        }
    }

    fn print_tasks(&self) {
        println!("Tasks:");
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {} ({})", index + 1, task.name, if task.completed { "Completed" } else { "Incomplete" });
        }
    }
}
