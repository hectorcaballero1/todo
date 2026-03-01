use crate::{storage, todo::Todo};

pub fn handle_create(name: &str) {
    if storage::exists(name) {
        eprintln!("Error: List '{}' already exists", name);
        return;
    }
    
    let list = Todo::new(name);
    if let Err(e) = storage::save(&list) {
        eprintln!("Error: {}", e);
        return;
    }
    println!("Created list: {}", name);
}

pub fn handle_add(name: &str, task: String) {
    let mut list = match storage::load(name) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    list.add(task);
    if let Err(e) = storage::save(&list) {
        eprintln!("Error: {}", e);
        return;
    }
    
    println!("Task added");
}

pub fn handle_next(name: &str) {
    let mut list = match storage::load(name) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    match list.next() {
        Some(item) => {
            println!("> {}", item);
            if let Err(e) = storage::save(&list) {
                eprintln!("Error: {}", e);
            }
        }
        None => println!("List is empty"),
    }
}

pub fn handle_list_items(name: &str) {
    match storage::load(name) {
        Ok(list) => println!("{}", list),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn handle_ls() {
    match storage::list_all() {
        Ok(names) => {
            if names.is_empty() {
                println!("No lists found");
            } else {
                println!("Your lists:");
                for name in names {
                    println!("  - {}", name);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn handle_remove(name: &str) {
    if !storage::exists(name) {
        eprintln!("Error: List '{}' does not exist", name);
        return;
    }

    if let Err(e) = storage::delete(name) {
        eprintln!("Error: {}", e);
        return;
    }

    println!("Deleted list: {}", name);
}

pub fn handle_remove_item(name: &str, index: usize) {
    let mut list = match storage::load(name) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if let Err(e) = list.remove(index) {
        eprintln!("Error: {}", e);
        return;
    }

    if let Err(e) = storage::save(&list) {
        eprintln!("Error: {}", e);
        return;
    }

    println!("Item {} removed", index);
}

pub fn print_usage() {
    println!("Usage:");
    println!("  todo create <name>          Create a spin list");
    println!("  todo ls                     List all lists");
    println!("  todo <name>                 Show list");
    println!("  todo <name> add <task>      Add task");
    println!("  todo <name> next            Get next task");
    println!("  todo rm <name>              Remove list");
    println!("  todo <name> rm <index>      Remove item by index");
}