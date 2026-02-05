use crate::todo::Todo;
use serde_json;
use std::{fs, io, path::PathBuf};

fn get_data_dir() -> PathBuf {
    PathBuf::from("./data")
}

pub fn save(list: &Todo) -> io::Result<()> {
    let dir = get_data_dir();
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(list)?;
    let file_path = dir.join(format!("{}.json", list.name()));
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

pub fn list_all() -> io::Result<Vec<String>> {
    let dir = get_data_dir();
    let mut names: Vec<String> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if let Some(name_without_suffix) = name.strip_suffix(".json") {
                names.push(name_without_suffix.to_string());
            }
        }
    }
    Ok(names)
}
