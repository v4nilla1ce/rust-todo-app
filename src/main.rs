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