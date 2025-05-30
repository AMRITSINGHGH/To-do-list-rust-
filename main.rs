use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const FILE_PATH: &str = "todo.txt";

fn main() {
    loop {
        println!("\nTo-Do List:");
        println!("1. Show tasks");
        println!("2. Add task");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => show_tasks(),
            "2" => add_task(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn show_tasks() {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        println!("No tasks yet.");
        return;
    }

    let file = OpenOptions::new()
        .read(true)
        .open(FILE_PATH)
        .expect("Unable to open file");

    let reader = BufReader::new(file);

    println!("Tasks:");
    for (index, line) in reader.lines().enumerate() {
        println!("{}. {}", index + 1, line.unwrap());
    }
}

fn add_task() {
    println!("Enter your task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Unable to open file");

    writeln!(file, "{}", task.trim()).expect("Unable to write task");
    println!("Task added!");
}
