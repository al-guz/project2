use std::{fs::{self, File}, io::{self, Result, Write}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChecklistItem {
    complete: bool,
    id: u32,
    name: String
}
pub struct Checklist {
    pub todo_list: Vec<ChecklistItem>,
}

impl ChecklistItem {
    //makes new struct of checklistItem.
    fn new(name: String, id: u32) -> Self {
        ChecklistItem {
            name: name,
            id: id,
            complete: false
        }
    }
}

impl Checklist {
    // i hate this
    //FINALLY FINISHED THIS IN 7/10/24 YESSSSSSS
    //i feel proud of old data being saved to the new run and stuff
    pub fn load_json_to_todo_list(&mut self) -> io::Result<()> {
        let f = fs::read_to_string("checklist.json")?;
        let output: Vec<ChecklistItem> = serde_json::from_str(&f)?;
        for checklist_item in output {
            let item = ChecklistItem::new(checklist_item.name, checklist_item.id);
            self.todo_list.push(item);
        }
        Ok(())
    }
    
    pub fn print_list(&self) {
        if self.todo_list.is_empty() {println!("the todo list is empty");}
        println!("-------------------------------------");
        for item in self.todo_list.iter() {
            println!("----------------");
            println!("id: {} name: {} isCompleted: {}", item.id, item.name, item.complete);
        }
    }

    pub fn mark_todo_complete(&mut self, id:u32) -> Option<()> {
        for items in &mut self.todo_list {
            if items.id == id {
                println!("marked {} complete", items.name);
                items.complete = true;
                Some(());
            }
        }
        None
    }
    
    pub fn add(&mut self, name: String) {
        println!("added {}", &name);
        let checklist_item = ChecklistItem::new(name, self.todo_list.len() as u32 + 1);
        self.todo_list.push(checklist_item);
    }

    //not need but i hate you and i can do whatever i want
    //even if it means to fuck up my pc (im dumb)
    pub fn new() -> Self {
        Self {
            todo_list: Vec::new(),
        }
    }

    //got this to work now
    // may be horrible but it gets the job done
    //and plus it gets me excited
    pub fn serialize_tasks(&self, mut file: &File) -> Result<()> {
        let mut real_json: Vec<u8> = Vec::new();
        //IM SORRY, but i feel proud so fuck you
        fn get_add(bytes: Box<Vec<u8>>, to: &mut Vec<u8>) {
            for b in bytes.iter() {
                to.push(b.to_owned());
            }
        }
        //json is pretty "gay" no offense to the gays
        let json = serde_json::to_vec_pretty(&self.todo_list)?;
        if !json.is_empty() {
            get_add(Box::new(json), &mut real_json);
            //still confused abt bytes and strings even tho theyre simple from the outside
            //smh
            file.write_all(&real_json)?;
        }

        Ok(())
    }
}