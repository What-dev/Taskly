# Taskly: a task list for the clinically insane

Taskly is a simple list creator that allows you to add, delete, list, or edit any of your big-brained tasks in one easy-to-use command line app! 

## Installation

#### Windows users can download the .exe and just run it wherever (you could even add it to your PATH I dont really care) this most likely will NOT be kept up to date (major releases only)

## Building
#### Download cargo using curl (You can also just go to rustup.rs) 

 ```
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

#### Download the source code or clone the repo, then in the Taskly directory run:

```bash
cargo build --release
```
## Updates
#### NEW FEATURE

- Implemented command line args for add (You can now do `taskly add task_name`replacing task_name with the name of your task)


- Inhanced the help menu and core UX

## TODO list:
- [ ] add task descriptions (might be tricky bc I suck at Rust)
- [ ] add task due dates (again, tricky, but both could use the same storage system so it might not be too bad)
- [ ] add task priority 
- [ ] add task tags
- [ ] make a neater looking menu
- [ ] add a way to view tasks by priority

## Closing Notes
#### This is my first Rust project, and it's tricky to balence Taskly's development with school and extra curriculars, so don't hold your breath for any of the TODO items.  
#### But if you see anything that could be improved, please let me know! I'm always looking to improve my code and learn new things.
