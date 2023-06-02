# Taskly V2: Electric Boogaloo

Taskly is a simple list creator that allows you to add and modify any of your big-brained tasks in one easy-to-use command line app! 

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
#### V2 is here with the help of Chat Gipity to teach me how to use serde!
- You can now delete tasks by their completion status!




## TODO list:
- [ ] add task descriptions (might be tricky bc I suck at Rust)
- [ ] add task due dates (again, tricky, but both could use the same storage system so it might not be too bad)
- [ ] add task priority 
- [ ] add task tags
- [ ] make a neater looking menu
- [ ] add a way to view tasks by priority
- [ ] gui using gtk-rs (maybe, idk how to use any gui library yet so we'll see)

## Closing Notes
#### V1 was converted into V2 in a little over an hour because everything was already set up, I just needed to learn how to use serde_json to pack it into the json file we use now!
## What's next?
#### everything in the todo list above since everything is now modular with a main Task struct that holds all the data for each task, so adding new features should be a breeze!

