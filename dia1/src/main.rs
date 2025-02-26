use item::Priority;
use item::Status;
use welcome::display_welcome_message;
use db::init_db;
use std::io::{self, stdin, Read, Write};

mod welcome;
mod db;
mod item;

/*
    Short title/headline. OK
    Detailed body/description. OK
    Add a new to-do item. OK
    List the title/headline of every to-do item. OK
    Display the body of a specific to-do item. OK
    Delete a to-do item. OK

*/

fn main() {
    
    // display welcome message
    display_welcome_message();

    // check/open database
    let db_name = "C:\\Temp\\Pollita.txt";
    let mut result = match init_db(db_name){
        Ok(r) => {
            println!("File opened at {}", db_name);
            r
        } 
        Err(e) => {
            print!("Error Initiating db: {}",e);
            return;
        }
    };

    // load items on db
    let mut items_item =  match db::read_db(&mut result) {
        Ok(items) => {
            println!("Loaded items successfully");
            items
        }
        Err(e) => { 
            println!("Found error {}", e);
            return;
        }
    };

    // main execution of the app
    println!("\n[+] Usage:\n");
    loop {
        print!("\n[1] Display all items\n[2] Delete item\n[3] Display item\n[4] Add Item\n[5] Exit\n\n");
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        match input.trim() {
            "1" => display_all_items(&mut items_item),
            "2" => delete_item(&mut items_item),
            "3" => display_item(&mut items_item),
            "4" => add_item(&mut items_item),
            "5" => break,
            _ => println!("Option Not Found"),
        }
    }

    // save and output
    let _ = match db::save_and_exit(items_item, &mut result) {
        Ok(_) => println!("Bye Bye!"),
        Err(e) => println!("Unexpected error: {}", e)
    };

}


fn display_all_items(items: &mut Vec<item::Item>) {
    for item in items {
        println!("title: {}, description: {}, status: {}, priority: {}", item.title, item.body, item.status, item.priority);
    }
}

fn delete_item(items: &mut Vec<item::Item>) {

    loop {
        print!("[+] Enter name to delete ('exit' to exit): ");
        let _ = io::stdout().flush();
        let mut name = String::new();
        let _ = io::stdin().read_line(&mut name);

        if name.trim_end().eq_ignore_ascii_case("exit") {
            break;
        } else {
            match items.iter().position(|x| x.title == name.trim_end()) {
                Some(pos) => {
                    items.remove(pos);
                    println!("Item Removed: {} at index {}", name, pos);
                },
                _ => {
                    println!("Item not Found: {}", name);
                }
            }
        }      
    }

}

fn add_item(items: &mut Vec<item::Item>) {
    print!("title > ");
    let _ = io::stdout().flush();
    let mut title = String::new();
    let _ = io::stdin().read_line(&mut title);

    print!("description > ");
    let _ = io::stdout().flush();
    let mut description = String::new();
    let _ = io::stdin().read_line(&mut description);

    print!("status > ");
    let _ = io::stdout().flush();
    let mut status = String::new();
    let _ = io::stdin().read_line(&mut status);

    print!("priority > ");
    let _ = io::stdout().flush();
    let mut priority = String::new();
    let _ = io::stdin().read_line(&mut priority);

    // add item
    items.push(item::Item::new(title.trim().to_string(), description.trim().to_string(), Status::from_str(status.trim()).unwrap(), Priority::from_str(priority.trim()).unwrap(), item::Added::New));

}

fn display_item(items: &mut Vec<item::Item>) {
    loop {
        println!("[1] Search by name\n[2] Search by priority\n[3] Search by status\n[4] Exit\n");
        print!("> ");
        let _ = io::stdout().flush();
        let mut option = String::new();
        let _ = io::stdin().read_line(&mut option);

        match option.trim() {
            // Search by Name
            "1" => {
                print!("name > ");
                let _ = io::stdout().flush();
                let mut name = String::new();
                let _ = io::stdin().read_line(&mut name);

                for item in items.iter() {
                    if item.title.trim().eq_ignore_ascii_case(name.trim_ascii_end()) {
                        item.display_item();
                    }
                }
            }

            // Search by Priority
            "2" => {
                print!("low / normal / urgent > ");
                let _ = io::stdout().flush();
                let mut prio = String::new();
                let _ = io::stdin().read_line(&mut prio);

                if let Some(user_priority) = Priority::from_str(prio.trim()) {
                    // Use `.iter()` to avoid moving `items`
                    for item in items.iter() {
                        if item.priority == user_priority {
                            item.display_item();
                        }
                    }
                } else {
                    println!("Invalid priority")
                }
            }

            // Search by Status
            "3" => {
                print!("not started / started / completed > ");
                let _ = io::stdout().flush();
                let mut status_input = String::new();
                let _ = io::stdin().read_line(&mut status_input);

                if let Some(user_status) = Status::from_str(status_input.trim()) {
                    for item in items.iter() {
                        if item.status == user_status {
                            item.display_item();
                        }
                    }
                } else {
                    println!("Invalid status")
                }
            },
            "4" => {
                break;
            }

            _ => {
                println!("No such option found");
            }
        }
    }
}