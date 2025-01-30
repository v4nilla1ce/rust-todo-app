# rust-todo-app

## Why this project?
The purpose of this project is to learn the basics of Rust.

Concepts i've learned:

	•	Vectors (Vec<Task>) to store tasks
	•	Structs (Task) to define task objects
	•	Enums (bool for done/not done)
	•	File handling with fs::write and fs::read_to_string
	•	Error handling using unwrap() and Option<>
	•	Loop & match expressions for user interaction

## Step by step:

### Step 1: Create a New Rust Project

Open your terminal and run:

```sh
cargo new todo_app
cd todo_app
```
### Step 2: Add Dependencies

Open Cargo.toml and add:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

	•	serde → Helps with serializing and deserializing data
	•	serde_json → Allows saving/loading tasks in JSON format

Run:
```sh
cargo build
```
### Step 3: Implement the To-Do List Logic

Open src/main.rs and replace it with this code:

```rust
use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Task { description, done: false }
    }
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\n📋 To-Do List:");
        println!("1️⃣ Add Task");
        println!("2️⃣ Show Tasks");
        println!("3️⃣ Mark Task as Done");
        println!("4️⃣ Exit");

        let choice = get_input("Choose an option: ");
        match choice.trim() {
            "1" => {
                let description = get_input("Enter task description: ");
                tasks.push(Task::new(description));
                save_tasks(&tasks);
            }
            "2" => show_tasks(&tasks),
            "3" => {
                show_tasks(&tasks);
                let index = get_input("Enter task number to mark as done: ");
                if let Ok(i) = index.trim().parse::<usize>() {
                    if i > 0 && i <= tasks.len() {
                        tasks[i - 1].done = true;
                        save_tasks(&tasks);
                        println!("✅ Task marked as done!");
                    } else {
                        println!("❌ Invalid task number!");
                    }
                }
            }
            "4" => {
                println!("👋 Exiting...");
                break;
            }
            _ => println!("❌ Invalid choice!"),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn show_tasks(tasks: &[Task]) {
    println!("\n📌 Tasks:");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.done { "✅" } else { "❌" };
        println!("{}. {} {}", index + 1, status, task.description);
    }
}

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}
```
### Step 4: Run the To-Do App

Compile and run:
```sh
cargo run
```

Example Usage:

```
📋 To-Do List:
1️⃣ Add Task
2️⃣ Show Tasks
3️⃣ Mark Task as Done
4️⃣ Exit
Choose an option: 1
Enter task description: Learn Rust

📋 To-Do List:
1️⃣ Add Task
2️⃣ Show Tasks
3️⃣ Mark Task as Done
4️⃣ Exit
Choose an option: 2

📌 Tasks:
1. ❌ Learn Rust
```

## Possible Improvements

	•	Improve error handling (e.g., handle invalid input)
	•	Add a feature to delete tasks
	•	Store tasks in a database (e.g., SQLite)
