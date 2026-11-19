use std::io::{Error, Write};
use crate::todos::format::{todo_display};
use crate::todos::structures::{TodoEnum, TodoCreate, TodoDeSerializer, TodoSerializer};
use crate::storage::{load_file};

pub fn view_todos(start: usize, offset: usize) {
    let results = load_file()
        .split("\n")
        .skip(start)
        .take(offset)
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();
    todo_display(results);
}

pub fn delete_todo_item(todo_id: String) {
    let contents = load_file();
    let result: Vec<&str> = contents
        .split('\n')
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return false;
            }
            let view = trimmed.to_string().serialize();
            view.get_id() != &todo_id
        })
        .collect();
    std::fs::write("enc_todos.txt", result.join("\n")).expect("Failed to write to file");
}

pub fn update_todo_item(todo_id: String, new_name: String) {
    let contents = load_file();
    let result: Vec<String> = contents
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut view = line.trim().to_string().serialize();
            if view.get_id() == &todo_id {
                view.set_name(new_name.clone());
            }
            view.deserialize()
        })
        .collect();
    std::fs::write("enc_todos.txt", result.join("\n")).expect("Failed to write to file");
}

pub fn retrieve_todo_item(todo_id: String) {
    let contents = load_file();
    let result: Vec<&str> = contents
        .split('\n')
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return false;
            }
            let view = trimmed.to_string().serialize();
            view.get_id() == &todo_id
        })
        .collect();
    if result.is_empty() {
        eprintln!("Todo not found");
        return;
    }
    todo_display(result.iter().map(|s| s.to_string()).collect());
}

pub fn add_todo_items(items: Vec<String>) -> Result<(), Error> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("enc_todos.txt")?;

    for item in items {
        let new_item = TodoCreate::new(item, TodoEnum::Todo);
        let deserialized = new_item.deserialize();
        writeln!(file, "{}", deserialized)?;
    }
    println!("File written to successfully!");
    Ok(())
}
