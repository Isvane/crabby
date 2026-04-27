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

fn main() {
    save_note("Testing my code", "notes.txt").expect("Failed to save note");
    println!("Note saved successfully!");
}
