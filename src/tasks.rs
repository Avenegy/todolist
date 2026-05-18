use crate::models::{Task, IDManager};
use std::collections::HashMap;


pub fn add_task(tasks: &mut HashMap<i64, Task>, name: String, next_id: &IDManager) {

    let id = next_id.get_id();

    tasks.insert(id, Task{name: name, done: false});
    println!("New task added!")
}

pub fn done_task(id: i64, tasks: &mut HashMap<i64, Task>) {
    match tasks.get_mut(&id) {
        Some(task) => {
            task.done = true;
            println!("Task done!")
        }, 
        None => println!("incorrect id!")
    }
}

pub fn remove_task(id: i64, tasks: &mut HashMap<i64, Task>){
    let removed_v = tasks.remove(&id);
    match removed_v {
        Some(task) => println!("Removed: {}", task.name),
        None => println!("ID not found!")
    }

}

pub fn edit_task(id: i64, tasks: &mut HashMap<i64, Task>, new_name: String){
        match tasks.get_mut(&id) {
            Some(task) => {
                task.name = new_name;
                println!("Name updated!")
            },
            None => println!("incorrect ID!")
    }
}


pub fn list_task(tasks: &HashMap<i64, Task>){

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