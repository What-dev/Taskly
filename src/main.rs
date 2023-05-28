#![allow(non_snake_case)]
use std::io::{self};
mod options;

fn main() {
    loop{
        println!("Welcome to Taskly! To get started, please input what you'd like to do, or press enter/return to exit! (add, list, or delete)");

        let mut menuinput = String::new();
        io::stdin()
            .read_line(&mut menuinput)
            .expect("Failed to read line");

        menuinput = menuinput.trim().to_string();

        if menuinput.is_empty() {

        }
        //TODO: add a way to edit tasks
        match menuinput.trim() {
            "add" => {
                options::add();
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