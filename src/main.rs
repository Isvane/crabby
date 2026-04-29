use std::fs::{self, OpenOptions};
use std::io::{self, Error, Write};

pub fn save_note(message: &str, file_path: &str) -> Result<(), Error> {
    let content = fs::read_to_string(file_path).unwrap_or_default();
    let id = content.lines().count() + 1;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}: {}", id, message)?;
    Ok(())
}

pub fn take_note(save_location: &str) -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    match input.to_lowercase().as_str() {
        "" => {
            println!("Please enter a valid note.");
        }
        _ => {
            save_note(input, save_location)?;
        }
    }
    Ok(())
}

pub fn read_note(file_path: &str) -> Result<(), Error> {
    let contents = fs::read_to_string(file_path)?;
    println!("File Contents:\n{}", contents);
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

fn main() {
    println!("Which file would you like to use? (e.g., diary.txt)");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read path");

    let path = path.trim();

    println!("\nEnter note: ");
    take_note(path).expect("Failed to take note");
    println!("Note saved successfully.");

    read_note(path).expect("Failed to read note");
    delete_note(path, 1).expect("Failed to delete note");
}
