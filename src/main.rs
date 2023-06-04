#![allow(non_snake_case)]
use std::io::{self};
use std::env;
use std::option::Option;
mod options;

fn main() {
    //TODO: Clean up arg parsing
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let cmd_input: Option<String> = if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        };
        match args[1].trim() {
            "add" => options::add(cmd_input),

            "edit" => options::edit(),

            "complete" => options::complete(cmd_input),

            "list" => options::list(cmd_input),

            "delete" => options::delete(),

            "help" => println!("Taskly Command Line Arguments:\n     add (optional input) - Add a task to your list\n     edit - Edit a task on your list\n     list - List all tasks on your list\n     complete (optional task number) - marks a task as completed \n     delete - Delete a task from your list\n     help - Display this help message"),

            _ => println!("Invalid argument! Please use one of the following: add, edit, list, or delete"),

        }
    } else {
        loop {
            println!("Welcome to Taskly! To get started, run Taskly help to see a list of commands.");

            let mut menuinput = String::new();
            io::stdin()
                .read_line(&mut menuinput)
                .expect("Failed to read line");

            menuinput = menuinput.trim().to_string();

            if menuinput.is_empty() {
                break;
            }

            match menuinput.trim() {
                "add" => {
                    options::add(None);
                }
                "edit" => {
                    options::edit();
                }
                "complete" => {
                    options::complete(None);
                }
                "list" => {
                    options::list(None);
                }
                "delete" => {
                    options::delete();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

