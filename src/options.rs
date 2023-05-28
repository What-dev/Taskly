use std::io::{self, Write};
use std::fs::OpenOptions;
use std::fs;

pub fn add() {
    println!("Input a task to be added to your list:");

    let folder_path = "Taskly";
    let file_name = "taskly.txt";
    let file_path = format!("{}/{}", folder_path, file_name);

    // Create the folder if it doesn't exist
    fs::create_dir_all(folder_path).expect("Failed to create folder.");

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .expect("Failed to open file.");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        if input.is_empty() {
            break;
        }

        writeln!(file, "{}", input).expect("Failed to write to file.");

        println!("Successfully added '{}', to your Taskly list!", input);

    }

    println!("File created successfully at: {}", file_path);
}

pub fn list() {
    let file_path = "Taskly/taskly.txt";
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("File contents:");
            println!("{}", contents);
    
        }
        Err(err) => {
            println!("Error reading file: {}", err);
    
        }
    }
}

pub fn delete() {
    let file_path = "Taskly/taskly.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
    
            return;
        }
    };

    println!("File contents:");
    println!("{}", contents);

    println!("Enter the number of the task you want to delete [or type 'bomb' to completely annihilate the taskly file >:)]:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "bomb" {
    match fs::remove_file(file_path) {
        Ok(()) => println!("File deleted successfully."),
        Err(err) => println!("Error deleting file: {}", err),
        }
    }
     else {

    let task_number: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
    
            return;
        }
    };

    let tasks: Vec<&str> = contents.lines().collect();
    if task_number >= 1 && task_number <= tasks.len() {
        let updated_tasks: Vec<&str> = tasks
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != task_number - 1)
            .map(|(_, &task)| task)
            .collect();

        let updated_content = updated_tasks.join("\n");
        match fs::write(file_path, updated_content) {
            Ok(_) => println!("Task deleted successfully!"),
            Err(err) => println!("Error deleting task: {}", err),
        }
    } else {
        println!("Invalid task number. Please enter a valid number.");
    }
    }
    
}