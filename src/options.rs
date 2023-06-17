use std::io::{self, Write};
use std::fs::OpenOptions;
use std::fs;
use std::option::Option;
use serde::{Serialize, Deserialize};
use std::path::Path;
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

pub fn add(add_input: Option<String>) {
    println!("Input a task to be added to your list:");

    let folder_path = "Taskly";
    let file_name = "taskly.json";
    let file_path = format!("{}/{}", folder_path, file_name);

    // Create the folder if it doesn't exist
    fs::create_dir_all(folder_path).expect("Failed to create folder.");

    let mut tasks: Vec<Task> = Vec::new();

    // Load existing tasks from the file if it exists
    if Path::new(&file_path).exists() {
        let file_content = fs::read_to_string(&file_path)
            .expect("Failed to read file.");

        tasks = serde_json::from_str(&file_content)
            .expect("Failed to parse file content as JSON.");
    }

    let mut input = String::new();
    if let Some(add_input_str) = add_input {
        if add_input_str.len() > 1 {
            input = add_input_str;
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

            let task = Task {
                name: input.clone(),
                completed: false,
            };

            tasks.push(task);

            println!("Successfully added '{}', to your Taskly list!", input);

            input.clear();
        }
    } else {
        let task = Task {
            name: input.clone(),
            completed: false,
        };

        tasks.push(task);

        println!("Successfully added '{}', to your Taskly list!", input);
    }

    // Save the updated tasks to the file
    let serialized_tasks = serde_json::to_string(&tasks)
        .expect("Failed to serialize tasks to JSON.");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)
        .expect("Failed to open file.");

    writeln!(file, "{}", serialized_tasks).expect("Failed to write to file.");

    println!("File created successfully at: {}", file_path);
}

pub fn list(list_input: Option<String>) {

    let mut input = String::new();
    if let Some(list_input_str) = list_input {
        if list_input_str.len() > 1 {
            input = list_input_str;
        }
    }

    if input.is_empty() {
        let file_path = "Taskly/taskly.json";
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                match serde_json::from_str::<Vec<Task>>(&contents) {
                    Ok(tasks) => {
                        println!("Your Taskly list:");
                        for (index, task) in tasks.iter().enumerate() {
                            println!("{}: {} - Completed: {}", index + 1, task.name, task.completed);
                        }
                    }
                    Err(err) => {
                        println!("Error parsing file content as JSON: {}", err);
                    }
                }
            }
            Err(err) => {
                println!("Error reading file: {}", err);
            }
        }
    }
    else if input == "uncompleted" {
        let file_path = "Taskly/taskly.json";
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                match serde_json::from_str::<Vec<Task>>(&contents) {
                    Ok(tasks) => {
                        println!("Your Taskly list:");
                        for (index, task) in tasks.iter().enumerate() {
                            if !task.completed {
                                println!("{}: {} - Completed: {}", index + 1, task.name, task.completed);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error parsing file content as JSON: {}", err);
                    }
                }
            }
            Err(err) => {
                println!("Error reading file: {}", err);
            }
        }
    }
    else if input == "completed" {
        let file_path = "Taskly/taskly.json";
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                match serde_json::from_str::<Vec<Task>>(&contents) {
                    Ok(tasks) => {
                        println!("Your Taskly list:");
                        for (index, task) in tasks.iter().enumerate() {
                            if task.completed {
                                println!("{}: {} - Completed: {}", index + 1, task.name, task.completed);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error parsing file content as JSON: {}", err);
                    }
                }
            }
            Err(err) => {
                println!("Error reading file: {}", err);
            }
        }
    }
}

pub fn complete(number: Option<String>) {
    let file_path = "Taskly/taskly.json";

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let mut tasks: Vec<Task> = match serde_json::from_str(&contents) {
        Ok(tasks) => tasks,
        Err(err) => {
            println!("Error parsing file contents as JSON: {}", err);
            return;
        }
    };

    println!("Your Taskly list:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {} - Completed: {} - Description: '{}'", index + 1, task.name, task.completed, task.desc);
    }

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

    if task_number >= 1 && task_number <= tasks.len() {
        tasks[task_number - 1].completed = true;

        let updated_content = serde_json::to_string_pretty(&tasks).unwrap();
        match fs::write(file_path, updated_content) {
            Ok(()) => println!("Task completed successfully."),
            Err(err) => println!("Error completing task: {}", err),
        }
    } else {
        println!("Invalid task number. Please enter a valid number.");
    }
}

pub fn delete() {
    let file_path = "Taskly/taskly.json";

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let mut tasks: Vec<Task> = match serde_json::from_str(&contents) {
        Ok(tasks) => tasks,
        Err(err) => {
            println!("Error parsing file contents as JSON: {}", err);
            return;
        }
    };

    println!("Your Taskly list:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {} - Completed: {}", index + 1, task.name, task.completed);
    }

    println!("Enter the number of the task you would like to delete, or remove all with 'bomb', completed tasks with 'completed', or uncompleted tasks with 'uncompleted':");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_lowercase();

    if input == "bomb" {
        match fs::remove_file(file_path) {
            Ok(()) => println!("File deleted successfully."),
            Err(err) => println!("Error deleting file: {}", err),
        }
    } else if input == "completed" {
        tasks.retain(|task| !task.completed);

        let updated_content = serde_json::to_string_pretty(&tasks).unwrap();
        match fs::write(file_path, updated_content) {
            Ok(()) => println!("Completed tasks deleted successfully!"),
            Err(err) => println!("Error deleting completed tasks: {}", err),
        }
    } else if input == "uncompleted" {
        tasks.retain(|task| task.completed);

        let updated_content = serde_json::to_string_pretty(&tasks).unwrap();
        match fs::write(file_path, updated_content) {
            Ok(()) => println!("Uncompleted tasks deleted successfully!"),
            Err(err) => println!("Error deleting uncompleted tasks: {}", err),
        }
    } else {
        let task_number: usize = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return;
            }
        };

        if task_number >= 1 && task_number <= tasks.len() {
            tasks.remove(task_number - 1);

            let updated_content = serde_json::to_string_pretty(&tasks).unwrap();
            match fs::write(file_path, updated_content) {
                Ok(()) => println!("Task deleted successfully!"),
                Err(err) => println!("Error deleting task: {}", err),
            }
        } else {
            println!("Invalid task number. Please enter a valid number.");
        }
    }
}

pub fn edit() {
    let file_path = "Taskly/taskly.json";

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let mut tasks: Vec<Task> = match serde_json::from_str(&contents) {
        Ok(tasks) => tasks,
        Err(err) => {
            println!("Error parsing file contents as JSON: {}", err);
            return;
        }
    };

    println!("Your Taskly list:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {} - Completed: {} - Description: '{}'", index + 1, task.name, task.completed, task.desc);
    }

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

    if task_number >= 1 && task_number <= tasks.len() {
        println!("Enter the new task:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let task_edited = input.trim().to_string();

        tasks[task_number - 1].name = task_edited;

        let updated_content = serde_json::to_string_pretty(&tasks).unwrap();
        match fs::write(file_path, updated_content) {
            Ok(()) => println!("Task edited successfully!"),
            Err(err) => println!("Error editing task: {}", err),
        }
    } else {
        println!("Invalid task number. Please enter a valid number.");
    }
}