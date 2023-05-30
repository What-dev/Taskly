#![allow(non_snake_case)]
use std::io::{self};
use std::env;
use std::option::Option;
mod options;

fn main() {
//add command line arguements for each menu option to jump straight to the option
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let cmd_input: Option<String> = if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        };
        match args[1].trim() {
            "add" => {
                if let Some(input) = cmd_input {
                    if input.len() > 1 {
                        options::add(Some(input));
                    } else {
                        options::add(None);
                    }
                } else {
                    options::add(None);
                }
                return;
            }
            "edit" => {
                options::edit();
            }
            "list" => {
                options::list();
                return;
            }
            "delete" => {
                options::delete();
            }
            "help" => {
                println!("Taskly Command Line Arguments:\n     add (optional input) - Add a task to your list\n     edit - Edit a task on your list\n     list - List all tasks on your list\n     delete - Delete a task from your list\n     help - Display this help message");
                return;
            }
            _ => {
                println!("Invalid argument! Please use one of the following: add, edit, list, or delete");
                return;
            }
        }
    }

    loop{
        println!("Welcome to Taskly! To get started, please input what you'd like to do, or press enter/return to exit! (add, edit, list, or delete)");

        let mut menuinput = String::new();
        io::stdin()
            .read_line(&mut menuinput)
            .expect("Failed to read line");

        menuinput = menuinput.trim().to_string();

        if menuinput.is_empty() {

        }

        match menuinput.trim() {
            "add" => {
                options::add(None);
            }
            "edit" => {
                options::edit();
            }
            "list" => {
                options::list();
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