use chrono::{self, DateTime, Local};
use std::fs::{self, OpenOptions};
use std::io::{self, Error, Write};
use std::path::PathBuf;

pub fn save_note(message: &str, file_path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let content = fs::read_to_string(file_path).unwrap_or_default();
    let id = content.lines().count() + 1;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}: {} [{}]", id, message, time)?;
    Ok(())
}

pub fn take_note(save_location: &str) -> Result<(), Error> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        match input.to_lowercase().as_str() {
            "quit" => break,
            "" => {
                println!("Please enter a valid note.");
            }
            _ => {
                save_note(input, save_location)?;
            }
        }
    }

    Ok(())
}

pub fn read_note(file_path: &str) -> Result<(), Error> {
    if !std::path::Path::new(file_path).exists() {
        println!("File not found.");
        return Ok(());
    }

    let contents = fs::read_to_string(file_path)?;

    if contents.is_empty() {
        println!("Notes empty")
    } else {
        println!("File Contents:\n{}", contents);
    }
    Ok(())
}

pub fn delete_note(file_path: &str, target_id: usize) -> Result<(), Error> {
    let content = fs::read_to_string(file_path)?;
    let mut new_content = String::new();
    let mut found = false;

    for line in content.lines() {
        if line.starts_with(&format!("{}:", target_id)) {
            found = true;
            continue;
        }
        new_content.push_str(line);
        new_content.push('\n');
    }

    if found {
        fs::write(file_path, new_content)?;
        println!("Note {} deleted.", target_id);
    } else {
        println!("Note with ID {} not found.", target_id);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    println!(" ---- CRABBY (WIP) ---- ");
    println!("Which file would you like to use? (default: diary.txt)");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read path");

    let path = path.trim();

    let mut path_buf = PathBuf::from(if path.is_empty() { "diary.txt" } else { path });

    if path_buf.extension().is_none() {
        path_buf.set_extension("txt");
    }

    let final_path = path_buf.to_string_lossy().into_owned();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "list" => read_note(&final_path)?,
            "add" => take_note(&final_path)?,
            "delete" => {
                println!("Enter the ID to delete:");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str)?;

                match id_str.trim().parse::<usize>() {
                    Ok(id) => delete_note(&final_path, id)?,
                    Err(_) => println!("Please enter a valid number!"),
                }
            }
            "quit" => break Ok(()),
            _ => println!("Invalid command"),
        }
    }
}
