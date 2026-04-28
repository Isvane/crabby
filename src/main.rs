use std::fs::{self, OpenOptions};
use std::io::{self, Error, Write};

pub fn save_note(message: &str, file_path: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", message)?;
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
}
