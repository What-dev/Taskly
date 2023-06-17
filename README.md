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
- added descriptions to tasks! (requires a new taskly.json file, please delete the old one using ```taskly delete```)




## TODO list:
- [ ] add task due dates
- [ ] add task priority 
- [ ] add task tags
- [ ] make a neater looking menu (tui-rs looks like a neat lib for this)
- [ ] add a way to view/sort tasks by priority
- [ ] add a way to view/sort tasks by tag

## Closing Notes
#### small update, nothing changed internally but with V2 this project actually has some sort of potential.
## What's next?
#### everything in the todo list above since everything is now modular with a main Task struct that holds all the data for each task, so adding new features should be a breeze!

