use chrono::{DateTime, Local};
use std::fs::{self, OpenOptions};
use std::io::{self, Error, Write};
use std::path::PathBuf;

fn prompt(message: &str) -> Result<String, Error> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn save_note(message: &str, file_path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "[{}] {}", time, message)?;
    println!("Note saved.");
    Ok(())
}

pub fn take_note(save_location: &str) -> Result<(), Error> {
    println!("--- Adding Notes (type 'quit' to exit) ---");
    loop {
        let input = prompt("add > ")?;

        match input.to_lowercase().as_str() {
            "quit" | "exit" => break,
            "" => continue,
            _ => save_note(&input, save_location)?,
        }
    }
    Ok(())
}

pub fn read_note(file_path: &str) -> Result<(), Error> {
    if !std::path::Path::new(file_path).exists() {
        println!("! No notes file found yet.");
        return Ok(());
    }

    let contents = fs::read_to_string(file_path)?;
    println!("\n--- Current Notes ({}) ---", file_path);

    if contents.trim().is_empty() {
        println!("(The file is empty)");
    } else {
        for (index, line) in contents.lines().enumerate() {
            println!("{:2} | {}", index + 1, line);
        }
    }
    println!("--------------------------");
    Ok(())
}

pub fn delete_note(file_path: &str) -> Result<(), Error> {
    read_note(file_path)?;

    let content = fs::read_to_string(file_path)?;
    if content.trim().is_empty() {
        return Ok(());
    }

    let input = prompt("Enter ID to delete: ")?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let id: usize = match input.parse() {
        Ok(num) if num > 0 && num <= lines.len() => num,
        _ => {
            println!("! Invalid ID.");
            return Ok(());
        }
    };

    lines.remove(id - 1);
    let mut final_content = lines.join("\n");
    if !final_content.is_empty() {
        final_content.push('\n');
    }

    fs::write(file_path, final_content)?;
    println!("Note #{} deleted.", id);

    Ok(())
}

fn main() -> Result<(), Error> {
    println!("==== CRABBY (v0.1.0) ====");

    let path_input = prompt("Select file (default: diary.txt): ")?;
    let mut path_buf = PathBuf::from(if path_input.is_empty() {
        "diary.txt"
    } else {
        &path_input
    });

    if path_buf.extension().is_none() {
        path_buf.set_extension("txt");
    }

    let final_path = path_buf.to_string_lossy().into_owned();
    println!(
        "Using: {}\nType 'list', 'add', 'delete', or 'quit'.",
        final_path
    );

    loop {
        let cmd = prompt(">> ")?.to_lowercase();

        match cmd.as_str() {
            "list" => read_note(&final_path)?,
            "add" => take_note(&final_path)?,
            "delete" => delete_note(&final_path)?,
            "quit" => {
                println!("Goodbye!");
                break Ok(());
            }
            _ => println!("! Unknown command. Try: list, add, delete, quit"),
        }
    }
}
