use chrono::{self, DateTime, Local};
use std::fs::{self, OpenOptions};
use std::io::{self, Error, Write};
use std::path::PathBuf;

pub fn save_note(message: &str, file_path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{} [{}]", message, time)?;
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
        for (index, line) in contents.lines().enumerate() {
            let id = index + 1;

            println!("{} {}", id.to_string(), line);
        }
    }
    Ok(())
}

pub fn delete_note(file_path: &str) -> Result<(), Error> {
    read_note(file_path)?;

    let content = fs::read_to_string(file_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    if content.trim().is_empty() {
        return Ok(());
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let id: usize = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("Invalid ID. Please enter a number.");
            return Ok(());
        }
    };

    if id > lines.len() {
        println!("ID out of range!");
        return Ok(());
    }

    lines.remove(id - 1);

    let after_delete = lines.join("\n");

    if !after_delete.is_empty() {
        std::fs::write(file_path, after_delete + "\n")?;
    } else {
        std::fs::write(file_path, "")?;
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
            "delete" => delete_note(&final_path)?,
            "quit" => break Ok(()),
            _ => println!("Invalid command"),
        }
    }
}
