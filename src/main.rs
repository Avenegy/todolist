use std::io;
use std::collections::HashMap;
use std::io::Write;

mod models;
use models::{Task, IDManager};
mod tasks;
use tasks::{add_task, remove_task, edit_task, list_task, done_task};
mod storage;
use storage::{load_tasks, save_task};

fn main() {
    println!("
 _____ ____  ____  ____    _     _  ____ _____ 
/__ __Y  _ \\/  _ \\/  _ \\  / \\   / \\/ ___Y__ __\\
  / \\ | / \\|| | \\|| / \\|  | |   | ||    \\ / \\  
  | | | \\_/|| |_/|| \\_/|  | |_/\\| |\\___ | | |  
  \\_/ \\____/\\____/\\____/  \\____/\\_/\\____/ \\_/
    ");
    println!("Type 'help' for commands");
    let mut active_task: HashMap<i64, Task>  = load_tasks();
    let start_id = active_task.keys().max().copied().unwrap_or(0) + 1;
    let id_manager = IDManager::new(start_id);
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut promt: String = String::from("");
        let _ = io::stdin().read_line(&mut promt);
        let promt = promt.to_lowercase();

        let parts: Vec<&str> = promt.split_whitespace().collect();
        if parts.is_empty() { continue; }
        let rest = &parts[1..];
        let name = rest.join(" ");
        match parts[0] {
            "add" => add_task(&mut active_task, name, &id_manager),
            "done" => done_task(match parts[1].trim().parse::<i64>(){
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid ID!");
                    continue;
                }
            }, &mut active_task),
            "remove" => remove_task(match parts[1].trim().parse::<i64>(){
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid ID!");
                    continue;
                }
            }, &mut active_task),
            "edit" => edit_task(match parts[1].trim().parse::<i64>(){
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid ID!");
                    continue;
                }
            }, &mut active_task, parts[2..].join(" ")),
            "help" => println!("Commands:\n Add new task - add [Task Name]\n Complete active task - done [Task ID]\n Remove active task - remove [Task ID]\n Edit active task - edit [Task ID] [New Name]"),
            "list" => list_task(&mut active_task),
            "exit" => break,
            _ => println!("Idk this command bro"),
        }
        save_task(&mut active_task)
    }
}