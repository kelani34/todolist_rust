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

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("1. Add task");
        println!("2. Mark task complete");
        println!("3. Print tasks");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter task name:");
                let mut task_name = String::new();
                io::stdin().read_line(&mut task_name).expect("Failed to read line");
                let task = Task {
                    name: task_name,
                    completed: false,
                };
                todo_list.add_task(task);
            }
            2 => {
                println!("Enter task name:");
                let mut task_name = String::new();
                io::stdin().read_line(&mut task_name).expect("Failed to read line");
                todo_list.mark_task_complete(task_name.trim());
            }
            3 => todo_list.print_tasks(),
            4 => break,
            _ => continue,
        }
    }
}
