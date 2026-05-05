use chrono::{DateTime, Local};
use colored::Colorize;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Error, Write};
use std::path::PathBuf;

fn prompt(message: &str) -> Result<String, Error> {
    print!("{}", message.bold().cyan());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn formatted_line(index: usize, line: &str) {
    if let Some(pos) = line.find(']') {
        let (ts, msg) = line.split_at(pos + 1);
        println!(
            "{:2} | {} {}",
            (index + 1).to_string().magenta(),
            ts.dimmed(),
            msg.white()
        );
    } else {
        println!("{:2} | {}", (index + 1).to_string().magenta(), line);
    }
}

pub fn save_note(message: &str, file_path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "[{}] {}", time, message)?;
    println!("{}", "✔ Note saved.".green());
    Ok(())
}

pub fn take_note(save_location: &str) -> Result<(), Error> {
    println!(
        "\n{}",
        "--- Adding Notes (type 'quit' to exit) ---".yellow().bold()
    );
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

pub fn show_stats(file_path: &str) -> Result<(), Error> {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("{}", "! No data found to analyze.".red());
            return Ok(());
        }
    };

    let reader = io::BufReader::new(file);
    let mut letters = 0;
    let mut words = 0;
    let mut line_count = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some((_, message)) = line.split_once(']') {
            line_count += 1;
            words += message.split_whitespace().count();
            letters += message.chars().filter(|c| c.is_alphanumeric()).count();
        }
    }

    if line_count > 0 {
        let avg_len = letters as f64 / line_count as f64;

        println!(
            "\n{}",
            "┌──────────────────────────────────────────┐".blue()
        );
        println!(
            "│ {:^40} │",
            "SYSTEM ANALYTICS REPORT".bold().bright_white()
        );
        println!("{}", "├──────────────────────────────────────────┤".blue());

        println!(
            "│ {:<18} │ {:>19} │",
            "Entries Count".white(),
            line_count.to_string().cyan().bold()
        );
        println!(
            "│ {:<18} │ {:>19} │",
            "Word Count".white(),
            words.to_string().cyan().bold()
        );
        println!(
            "│ {:<18} │ {:>19} │",
            "Avg. Density".white(),
            format!("{:.1}", avg_len).cyan().bold()
        );

        println!("{}", "└──────────────────────────────────────────┘".blue());
    } else {
        println!("\n{} {}", "STATUS:".yellow().bold(), "Notebook is empty.");
    }

    Ok(())
}

pub fn read_note(file_path: &str) -> Result<(), Error> {
    if !std::path::Path::new(file_path).exists() {
        println!("{}", "! No notes file found.".red());
        return Ok(());
    }

    let contents = fs::read_to_string(file_path)?;
    println!(
        "\n{}",
        format!("--- Current Notes ({}) ---", file_path)
            .blue()
            .bold()
    );

    if contents.trim().is_empty() {
        println!("{}", "(The file is empty)".truecolor(128, 128, 128));
    } else {
        for (index, line) in contents.lines().enumerate() {
            formatted_line(index, line);
        }
    }
    println!("{}", "--------------------------".blue().bold());
    Ok(())
}

pub fn search_note(file_path: &str) -> Result<(), Error> {
    let query = prompt("Search for: ")?.to_lowercase();
    if query.is_empty() {
        return Ok(());
    }

    if !std::path::Path::new(file_path).exists() {
        println!("{}", "! No notes file found.".red());
        return Ok(());
    }

    let contents = fs::read_to_string(file_path)?;
    let matches: Vec<(usize, &str)> = contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .collect();

    if matches.is_empty() {
        println!(
            "{}",
            format!("! No matches found for '{}'.", query).yellow()
        );
    } else {
        println!(
            "\n{}",
            format!("--- Found {} Match(es) ---", matches.len())
                .green()
                .bold()
        );

        for (index, line) in matches {
            formatted_line(index, line);
        }
        println!("{}", "--------------------------".green().bold());
    }

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
            println!("{}", "! Invalid ID.".red().bold());
            return Ok(());
        }
    };

    lines.remove(id - 1);
    let mut final_content = lines.join("\n");
    if !final_content.is_empty() {
        final_content.push('\n');
    }

    fs::write(file_path, final_content)?;
    println!("{}", format!("Note #{} deleted.", id).red());
    read_note(file_path)?;

    Ok(())
}

fn main() -> Result<(), Error> {
    println!(
        "{}",
        "==== CRABBY (v0.1.0) ====".truecolor(255, 69, 0).bold()
    );

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
        "{} {}\n{} {}",
        "Target:".bright_black(),
        final_path.underline(),
        "Commands:".bright_black(),
        "list, add, search, delete, stats, clear, quit".yellow()
    );

    loop {
        let cmd = prompt(">> ")?.to_lowercase();

        match cmd.as_str() {
            "list" => read_note(&final_path)?,
            "add" => take_note(&final_path)?,
            "search" => search_note(&final_path)?,
            "delete" => delete_note(&final_path)?,
            "quit" => {
                println!("{}", "Goodbye!".green());
                break Ok(());
            }
            "clear" => {
                if let Err(e) = clearscreen::clear() {
                    eprintln!("{} {}", "! Could not clear screen:".red(), e);
                }
            }
            "stats" => show_stats(&final_path)?,
            _ => println!(
                "{} {}. Try: {}",
                "! Unknown command".red(),
                cmd.white().bold(),
                "list, add, delete, quit, clear".yellow()
            ),
        }
    }
}
