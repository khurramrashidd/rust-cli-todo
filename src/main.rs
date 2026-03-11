use std::io::{self, Write};
use std::fs;

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("\n===== TODO APP =====");
        println!("1. Show Tasks");
        println!("2. Add Task");
        println!("3. Delete Task");
        println!("4. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => show_tasks(&tasks),
            "2" => {
                let task = add_task();
                tasks.push(task);
                save_tasks(&tasks);
            }
            "3" => {
                delete_task(&mut tasks);
                save_tasks(&tasks);
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}

fn show_tasks(tasks: &Vec<String>) {
    if tasks.is_empty() {
        println!("No tasks yet.");
    } else {
        println!("\nYour Tasks:");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}

fn add_task() -> String {
    println!("Enter new task:");

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    task.trim().to_string()
}

fn delete_task(tasks: &mut Vec<String>) {
    show_tasks(tasks);

    println!("Enter task number to delete:");

    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();

    let index: usize = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    if index == 0 || index > tasks.len() {
        println!("Task not found.");
    } else {
        tasks.remove(index - 1);
        println!("Task deleted.");
    }
}

fn load_tasks() -> Vec<String> {
    match fs::read_to_string("tasks.txt") {
        Ok(content) => content.lines().map(|s| s.to_string()).collect(),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<String>) {
    let data = tasks.join("\n");
    fs::write("tasks.txt", data).unwrap();
}