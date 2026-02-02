use std::{fs, io, path::PathBuf};
use serde_json;
use crate::todo::Todo;

fn get_data_dir() -> PathBuf {
    PathBuf::from("./data")
}

pub fn save(list: &Todo) -> io::Result<()> {
    let json = serde_json::to_string(list)?;
    let file_path = get_data_dir().join(format!("{}.json", list.name()));
    fs::write(file_path, json)?;
    Ok(())
}

pub fn load(name: &str) -> io::Result<Todo> {
    let file_path = get_data_dir().join(format!("{}.json", name));
    let content = fs::read_to_string(file_path)?;
    let list: Todo = serde_json::from_str(&content)?;
    Ok(list)
}

pub fn exists(name: &str) -> bool {
    let file_path = get_data_dir().join(format!("{}.json", name));
    file_path.exists()
}