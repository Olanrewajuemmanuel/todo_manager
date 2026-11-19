use crate::todos::structures::{TodoSerializer, TodoView};

pub fn todo_display(dirty: Vec<String>) {
    let serialized: Vec<TodoView> = dirty
        .iter()
        .map(|line| line.trim().to_string().serialize())
        .collect();

    println!("{:<40} {:<20} {:<8} DATE", "ID", "NAME", "STATUS");
    println!("{}", "-".repeat(80));
    for todo in &serialized {
        println!(
            "{:<40} {:<20} {:<8} {}",
            todo.get_id(),
            todo.get_name(),
            format!("{:?}", todo.get_status()),
            todo.get_date_created()
        );
    }
}
