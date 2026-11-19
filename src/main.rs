//! TODO Manager CLI

fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo_manager::ui::run()?;
    Ok(())
}

