use std::fmt::write;
use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};
use std::io::BufReader;
use crate::item;


pub fn init_db(db_name: &str) -> Result<File, io::Error> {
    // create file if not created already
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(db_name);

    file
}

pub fn read_db(db_file: &mut File) -> Result<Vec<item::Item>, io::Error>{

    // read the contents of the file and create the vector to allocate them
    let reader = BufReader::new(db_file);
    let mut items = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        // skip invalid lines
        if parts.len() != 4 {
            continue;
        }
        let title = parts[0].trim().to_string();
        let description = parts[1].trim().to_string();
        let status: item::Status = item::Status::from_str(parts[2].trim()).expect("Could Not Found Status");
        let priority: item::Priority = item::Priority::from_str(parts[3].trim()).expect("Could Not Found Priority");
        let new = 0;

        // debugging purposes
        //println!("title: {}, description: {}, status: {}, priority: {}", title, description, status, priority);

        items.push(item::Item::new(
            title,
            description,
            status,
            priority,
            item::Added::from_str("0").expect("Not Added")
        ));
    }

    Ok(items)


}


pub fn save_and_exit(items: Vec<item::Item>, db_file: &mut File) -> std::io::Result<()> {
    for item in items {
        if item.is_new == item::Added::New {
            writeln!(db_file, "\n{}, {}, {}, {}", item.title, item.body, item.status, item.priority)?;
        } else {
            continue;
        }
    }
    Ok(())
}