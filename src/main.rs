use std::fs::OpenOptions;
use std::io::{Error, Write};

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
    std::io::stdin().read_line(&mut input)?;
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

fn main() {
    println!("Enter note: ");
    take_note("test.txt").expect("Failed to take note");
    println!("Note saved successfully");
}
