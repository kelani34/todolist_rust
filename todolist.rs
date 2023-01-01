use std::io;

struct Task {
    name: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}
