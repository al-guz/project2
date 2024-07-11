mod checklist;
//vs code extensions does the importing for me so i dont have to
//everything has to need a sacrifice and in this case their sanities
//shout out to the fallen soldiers
use std::fs::File;

use checklist::Checklist;

fn main() {
    let mut checklist = Checklist::new();
    if let Err(err) = checklist.load_json_to_todo_list() {
        println!("An error has occured while getting old data. {}", err);
    }
    let checklist_json = File::create("checklist.json").unwrap();
    
    //loops >:D
    loop {
        println!("---------------------------");
        println!("1 - add todo");
        println!("2 - list todos");
        println!("3 - mark todo complete");
        println!("4 - clear todo list");
        println!("5 - quit");
        
        let mut input = String::new();
        
        std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
        
        //couldve used clap for like the hard work and copy and pasting
        // but i digress
        match input.trim() {
            "1" => {
                println!("enter a name for todo");
                
                let mut name_input = String::new();
                
                std::io::stdin()
                .read_line(&mut name_input)
                .expect("Failed to read input");
                
                checklist.add(name_input);
            },
            "2" => checklist.print_list(),
            "3" => {
                println!("enter a name to mark complete");
                
                let mut id_input: String = String::new();
                
                std::io::stdin()
                .read_line(&mut id_input)
                .expect("Failed to read input");

                let id = id_input.trim().parse::<u32>().expect("Invalid ID");

                checklist.mark_todo_complete(id);
            }
            "4" => {
                checklist.todo_list.clear();
                println!("cleared todo list");
            },
            "5" => break,
            _ => println!("Failed to read input")
        }
    }

    //i love error handling <3
    match checklist.serialize_tasks(&checklist_json) {
        Ok(_) => println!("Saved tasks"),
        Err(e) => println!("error has occured : {}", e)
    }
}