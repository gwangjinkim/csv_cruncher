#[derive(Debug)]
struct Task {
    name: String,
    priority: i32,
}

impl Task {
    fn new(name: &str, priority: i32) -> Task {
        Task {
            name: String::from(name),
            priority,
        }
    }

    fn is_urgent(&self) -> bool {
        self.priority >= 3
    }
}

trait Filterable {
    fn is_urgent(&self) -> bool;
}

impl Filterable for Task {
    fn is_urgent(&self) -> bool {
        self.priority >= 3
    }
}

fn main() {
    let tasks = vec![
        Task::new("Code", 1),
        Task::new("Test", 3),
        Task::new("Deploy", 4),
    ];
    let urgent: Vec<&Task> = tasks.iter().filter(|t| t.is_urgent()).collect();
    println!("Urgent tasks: {:?}", urgent);
}

