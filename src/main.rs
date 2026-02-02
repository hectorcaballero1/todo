mod todo;
mod storage;

use todo::Todo;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage:");
        println!("  todo create <name>           Create a spin list");
        println!("  todo <name>                  Open list interactively");
        println!("  todo <name> next             Get next item from spin");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "create" => {
            if args.len() < 3 {
                println!("Usage: todo create <name>");
                return;
            }

            let name = &args[2];

            // Se verifica que no exista una lista con el mismo nombre
            if storage::exists(name) {
                println!("Error: List '{}' already exists", name);
                return;
            }

            // Creamos y persistimos la lista
            let list = Todo::new(name);
            if let Err(e) = storage::save(&list) {
                eprintln!("Error: {}", e);
                return;
            }
            println!("Created list: {}", name);
        }
        _ => {
            let name = &args[1];

            // Se verifica que exista la lista
            if !storage::exists(name) {
                println!("Error: List '{}' not found", name);
                return;
            }

            // Si no se ingresa un comando, se imprime la lista
            if args.len() < 3 {
                match storage::load(name) {
                    Ok(list) => println!("{}", list),
                    Err(e) => eprintln!("Error: {}", e),
                }
                return;
            }

            let subcommand = &args[2];

            match subcommand.as_str() {
                "add" => {
                    if args.len() < 4 {
                        println!("Usage: todo <name> add <task>");
                        return;
                    }
                    
                    let mut list = match storage::load(name) {
                        Ok(l) => l,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            return;
                        }
                    };
                    
                    list.add(args[3].clone());
                    
                    if let Err(e) = storage::save(&list) {
                        eprintln!("Error: {}", e);
                        return;
                    }
                    println!("Task added");
                }
                "next" => {
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
                                return; 
                            }
                        }
                        None => println!("List is empty"),
                    }
                    
                }
                _ => {
                    println!("Unknown command: {}", subcommand);
                }
            }
        }
    }
}