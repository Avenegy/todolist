use std::io;
use std::collections::HashMap;

fn main() {
    println!("TODO LIST");
    println!("Commands: add [Task Name], done [Task ID], remove [Task ID], edit [Task ID]");

    // let commands = vec![
    //     "add",
    //     "done",
    //     "remove",
    //     "edit",
    // ];

    let mut active_task: HashMap<i32, Task>  = HashMap::new();

    loop {
        let mut promt: String = String::from("");
        let _ = io::stdin().read_line(&mut promt);
        

        let parts: Vec<&str> = promt.split_whitespace().collect();
        let rest = &parts[1..];
        let name = rest.join(" ");

        match parts[0] {
            "add" => add_task(&mut active_task, name),
            "done" => done_task(),
            "remove" => remove_task(),
            "edit" => edit_task(),
            "list" => list_task(&mut active_task),
            "exit" => break,
            _ => println!("Idk this command bro"),
        }
    }
}

struct Task {
       name: String,
       done: bool,
    }

fn add_task(tasks: &mut HashMap<i32, Task>, name: String) {
    let id_unsize = &tasks.len() + 1;
    let id = id_unsize as i32;
    tasks.insert(id, Task{name: name, done: false});
}

fn done_task(){

}

fn remove_task(){

}

fn edit_task(){

}


fn list_task(tasks: &HashMap<i32, Task>){
    for (id, val) in tasks {
        let mut mark_done = String::from("[ ]");
        let name = &val.name;
        if val.done{
           mark_done = String::from("[x]");
        }
        println!("{id}. {name} {mark_done}");
    }

}