use std::io;

fn main() {
    println!("TODO LIST");
    println!("Commands: add [Task Name], done [Task ID], remove [Task ID], edit [Task ID]");

    // let commands = vec![
    //     "add",
    //     "done",
    //     "remove",
    //     "edit",
    // ];

    struct Tasks {
        name: String,
        done: bool,
        id: i32,
    }

    let mut promt: String = String::from("");
    let _ = io::stdin().read_line(&mut promt);
    let trimmed = promt.trim();
    
    match trimmed {
        "add" => add_task(),
        "done" => done_task(),
        "remove" => remove_task(),
        "edit" => edit_task(),
        _ => println!("Idk this command bro")
    }
}

fn add_task(){
    
}

fn done_task(){

}

fn remove_task(){

}

fn edit_task(){

}