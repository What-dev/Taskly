use std::io::{self, Write};
use std::fs::OpenOptions;
use std::fs;
use std::option::Option;


pub fn add(cmd_input: Option<String>) {
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

    let mut input = String::new();
    if let Some(cmd_input_str) = cmd_input {
        if cmd_input_str.len() > 1 {
            input = cmd_input_str;
        }
    }

    if input.is_empty() {
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_string();

            if input.is_empty() {
                break;
            }

            writeln!(file, "{}", input).expect("Failed to write to file.");

            println!("Successfully added '{}', to your Taskly list!", input);

            input.clear();
        }
    } else {
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

pub fn complete(number: Option<String>) {
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

    let task_number: usize = match number {
        Some(cmd_input_str) => match cmd_input_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return;
            }
        },
        None => {
            let mut input = String::new();
            println!("Enter the number of the task you want to complete:");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    return;
                }
            }
        }
    };

    let tasks: Vec<String> = contents.lines().map(|task| task.to_string()).collect();
    if task_number >= 1 && task_number <= tasks.len() {
        let updated_tasks: Vec<String> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                if i == task_number - 1 {
                    format!("\x1B[9m{}\x1B[0m - Completed", task)
                } else {
                    task.to_string()
                }
            })
            .collect();

        let updated_content = updated_tasks.join("\n");
        match fs::write(file_path, updated_content) {
            Ok(()) => println!("Task completed successfully."),
            Err(err) => println!("Error completing task: {}", err),
        }
    } else {
        println!("Invalid task number. Please enter a valid number.");
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

pub fn edit() {
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

    println!("Enter the number of the task you want to edit:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let task_number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");

                return;
            }
        };
        //add a function that takes an input and stores it in task_edited
        println!("Enter the new task:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let task_edited = input.trim().to_string();

        let tasks: Vec<&str> = contents.lines().collect();
        if task_number >= 1 && task_number <= tasks.len() {
            let updated_tasks: Vec<String> = tasks
                .iter()
                .enumerate()
                .map(|(i, &task)| {
                    if i == task_number - 1 {
                        // Modify the task here
                        task_edited.to_string()
                    } else {
                        task.to_string()
                    }
                })
                .collect();

            let updated_content = updated_tasks.join("\n");
            match fs::write(file_path, updated_content) {
                Ok(_) => println!("Task edited successfully!"),
                Err(err) => println!("Error editing task: {}", err),
            }
        } else {
            println!("Invalid task number. Please enter a valid number.");
        }
}