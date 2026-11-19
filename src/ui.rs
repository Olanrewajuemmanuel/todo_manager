use std::io::{self, Write};

use crate::commands::{add_todo_items, delete_todo_item, retrieve_todo_item, update_todo_item, view_todos};

#[derive(Debug)]
pub enum ApplicationError {
    Io(io::Error),
    Database(String),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Io(e) => write!(f, "IO error: {e}"),
            ApplicationError::Database(e) => write!(f, "Database error: {e}"),
        }
    }
}

impl std::error::Error for ApplicationError {}

fn display_welcome_message() {
    println!(
        r#"
 _____ ___  ___   ___
|_   _/ _ \|   \ / _ \
  | || (_) | |) | (_) |
  |_| \___/|___/ \___/  manager

  Usage: todo_manager <command> [args]
  Commands: add, view, update, delete
"#
    );
}

pub fn read_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_lowercase()
}

fn prompt(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();
    read_input()
}

fn parse_usize(value: &str, label: &str) -> Option<usize> {
    match value.parse::<usize>() {
        Ok(v) => Some(v),
        Err(e) => {
            match e.kind() {
                std::num::IntErrorKind::PosOverflow => eprintln!("{label} value is too large."),
                std::num::IntErrorKind::NegOverflow => eprintln!("{label} value is too small."),
                _ => eprintln!("Invalid {label} value."),
            }
            None
        }
    }
}

fn process_add() -> Result<(), ApplicationError> {
    let items: Vec<String> = prompt("Items (comma-separated): ")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    add_todo_items(items).map_err(|e| ApplicationError::Database(e.to_string()))
}

fn process_view() {
    let input = prompt("Enter start and limit (comma-separated e.g., 1,5): ");
    let parts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        eprintln!("Invalid input. Please provide two values separated by a comma.");
        return;
    }
    let start = match parse_usize(parts[0], "Start") {
        Some(v) => v.saturating_sub(1),
        None => return,
    };
    let offset = match parse_usize(parts[1], "Limit") {
        Some(v) => v,
        None => return,
    };
    view_todos(start, offset);
}

fn process_retrieve() {
    let id = prompt("Todo ID: ");
    retrieve_todo_item(id);
}

fn process_update() {
    let id = prompt("Todo ID: ");
    let name = prompt("New name: ");
    update_todo_item(id, name);
}

fn process_delete() {
    let id = prompt("Todo ID: ");
    delete_todo_item(id);
}

pub fn run() -> Result<(), ApplicationError> {
    display_welcome_message();
    loop {
        print!("\n[A] Add  [V] View  [R] Retrieve  [U] Update  [D] Delete  [Q] Quit\n> ");
        io::stdout().flush().unwrap();

        let input = read_input();

        match input.as_str() {
            "a" => process_add()?,
            "v" => process_view(),
            "r" => process_retrieve(),
            "u" => process_update(),
            "d" => process_delete(),
            "q" => break,
            _ => eprintln!("Unknown option: '{input}'"),
        }
    }
    Ok(())
}
