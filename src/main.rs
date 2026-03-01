mod commands;
mod storage;
mod todo;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        commands::print_usage();
        return;
    }

    match args[1].as_str() {
        "create" => {
            if args.len() < 3 {
                println!("Usage: todo create <name>");
                return;
            }
            commands::handle_create(&args[2]);
        }
        
        "ls" => {
            commands::handle_ls();
        }

        "rm" => {
            if args.len() < 3 {
                println!("Usage: todo rm <name>");
                return;
            }
            commands::handle_remove(&args[2]);
        }
        
        _ => {
            let name = &args[1];
            
            if !storage::exists(name) {
                eprintln!("Error: List '{}' not found", name);
                return;
            }
            
            if args.len() < 3 {
                commands::handle_list_items(name);
                return;
            }
            
            match args[2].as_str() {
                "add" => {
                    if args.len() < 4 {
                        println!("Usage: todo <name> add <task>");
                        return;
                    }
                    commands::handle_add(name, args[3].clone());
                }
                
                "next" => {
                    commands::handle_next(name);
                }

                "rm" => {
                    if args.len() < 4 {
                        println!("Usage: todo <name> rm <index>");
                        return;
                    }
                    match args[3].parse::<usize>() {
                        Ok(index) => commands::handle_remove_item(name, index),
                        Err(_) => eprintln!("Error: index must be a number"),
                    }
                }

                _ => {
                    eprintln!("Unknown command: {}", args[2]);
                }
            }
        }
    }
}
