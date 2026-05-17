use std::io;
use std::collections::HashMap;
use std::cell::{Cell};
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

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
    let id_manager = IDManager::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut promt: String = String::from("").to_lowercase();
        let _ = io::stdin().read_line(&mut promt);
        

        let parts: Vec<&str> = promt.split_whitespace().collect();
        if parts.is_empty() { continue; }
        let rest = &parts[1..];
        let name = rest.join(" ");
        match parts[0] {
            "add" => add_task(&mut active_task, name, &id_manager),
            "done" => done_task(parts[1].trim().parse::<i64>().unwrap(), &mut active_task),
            "remove" => remove_task(parts[1].trim().parse::<i64>().unwrap(), &mut active_task),
            "edit" => edit_task(parts[1].trim().parse::<i64>().unwrap(), &mut active_task, parts[2..].join(" ")),
            "help" => println!("Commands:\n Add new task - add [Task Name]\n Complete active task - done [Task ID]\n Remove active task - remove [Task ID]\n Edit active task - edit [Task ID] [New Name]"),
            "list" => list_task(&mut active_task),
            "exit" => break,
            _ => println!("Idk this command bro"),
        }
        save_task(&mut active_task)
    }
}


#[derive(Serialize, Deserialize)]
struct Task {
       name: String,
       done: bool,
    }


pub struct IDManager {
  next_id: Cell<i64>
}

impl IDManager {
  pub fn new() -> IDManager {
    IDManager { next_id: Cell::new(1) }
  }

  pub fn get_id(&self) -> i64 {
    let ans = self.next_id.get(); 
    self.next_id.set(ans + 1); 
    ans
  }
}

fn add_task(tasks: &mut HashMap<i64, Task>, name: String, next_id: &IDManager) {

    let id = next_id.get_id();

    tasks.insert(id, Task{name: name, done: false});
    println!("New task added!")
}

fn done_task(id: i64, tasks: &mut HashMap<i64, Task>) {
    match tasks.get_mut(&id) {
        Some(task) => {
            task.done = true;
            println!("Task done!")
        }, 
        None => println!("incorrect id!")
    }
}

fn remove_task(id: i64, tasks: &mut HashMap<i64, Task>){
    let removed_v = tasks.remove(&id);
    match removed_v {
        Some(task) => println!("Removed: {}", task.name),
        None => println!("Id not found")
    }

}

fn edit_task(id: i64, tasks: &mut HashMap<i64, Task>, new_name: String){
        match tasks.get_mut(&id) {
            Some(task) => {
                task.name = new_name;
                println!("Name updated!")
            },
            None => println!("incorrect id!")
    }
}


fn list_task(tasks: &HashMap<i64, Task>){

    println!("TASK LIST:");
    for (id, val) in tasks {
        let mut mark_done = String::from("[ ]");
        let name = &val.name;
        if val.done{
           mark_done = String::from("[x]");
        }
        println!("{id}. {name} {mark_done}");
    }
}

fn save_task(tasks: &HashMap<i64, Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}

fn load_tasks() -> HashMap<i64, Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap(), 
        Err(_) => HashMap::new()
    }
}