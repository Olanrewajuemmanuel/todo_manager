use std::fs::read_to_string;

pub fn load_file() -> String {
    const DBPATH: &str = "enc_todos.txt";
    read_to_string(DBPATH).expect("[Unrecoverable error]: DB missing!")
}

