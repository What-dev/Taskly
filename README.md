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

- Added the complete option, which allows you to mark a task as complete using **Taskly complete (the number of the task)**.


## TODO list:
- [ ] add task descriptions (might be tricky bc I suck at Rust)
- [ ] add task due dates (again, tricky, but both could use the same storage system so it might not be too bad)
- [ ] add task priority 
- [ ] add task tags
- [ ] make a neater looking menu
- [ ] add a way to view tasks by priority
- [ ] gui using gtk-rs (maybe, idk how to use any gui library yet so we'll see)

## Closing Notes
#### I'm planning a bigger update now that basic features are implemented to the back-end that will utilize json to store data more efficiently than txt files that were simple when I didn't know much.
### TLDR: Taskly v2: Electric Boogaloo
