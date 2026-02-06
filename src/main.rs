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
                
                _ => {
                    eprintln!("Unknown command: {}", args[2]);
                }
            }
        }
    }
}
