use chrono::{DateTime, Local};
use colored::Colorize;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Error, Write};

use crate::utils::{formatted_line, prompt};

pub struct Notebook {
    file_path: String,
}

impl Notebook {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn save_note(&self, message: &str) -> Result<(), Error> {
        let now: DateTime<Local> = Local::now();
        let time = now.format("%Y-%m-%d %H:%M:%S");

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        writeln!(file, "[{}] {}", time, message)?;
        println!("{}", "Note saved.".green());
        Ok(())
    }

    pub fn take_note(&self) -> Result<(), Error> {
        println!(
            "\n{}",
            "--- Adding Notes (type 'quit' to exit) ---".yellow().bold()
        );
        loop {
            let input = prompt("add > ")?;

            match input.to_lowercase().as_str() {
                "quit" | "exit" => break,
                "" => continue,
                _ => self.save_note(&input)?,
            }
        }
        println!("");
        Ok(())
    }

    pub fn show_stats(&self) -> Result<(), Error> {
        let file = match File::open(&self.file_path) {
            Ok(f) => f,
            Err(_) => {
                println!("\n{}", "! No data found to analyze.".red());
                println!("");
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
            println!("\n{} Notebook is empty.", "STATUS:".yellow().bold());
        }
        println!("");
        Ok(())
    }

    pub fn read_note(&self) -> Result<(), Error> {
        if !std::path::Path::new(&self.file_path).exists() {
            println!("\n{}", "! No notes file found.".red());
            println!("");
            return Ok(());
        }

        let contents = fs::read_to_string(&self.file_path)?;
        println!(
            "\n{}",
            format!("--- Current Notes ({}) ---", self.file_path)
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
        println!("");
        Ok(())
    }

    pub fn search_note(&self) -> Result<(), Error> {
        println!("");
        let query = prompt("Search for: ")?.to_lowercase();

        if query.is_empty() {
            println!("");
            return Ok(());
        }

        if !std::path::Path::new(&self.file_path).exists() {
            println!("\n{}", "! No notes file found.".red());
            println!("");
            return Ok(());
        }

        let contents = fs::read_to_string(&self.file_path)?;
        let matches: Vec<(usize, &str)> = contents
            .lines()
            .enumerate()
            .filter(|(_, line)| line.to_lowercase().contains(&query))
            .collect();

        if matches.is_empty() {
            println!(
                "\n{}",
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
        println!("");
        Ok(())
    }

    pub fn delete_note(&self) -> Result<(), Error> {
        self.read_note()?;

        let content = fs::read_to_string(&self.file_path)?;
        if content.trim().is_empty() {
            return Ok(());
        }

        let input = prompt("Enter ID to delete: ")?;
        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        let id: usize = match input.parse() {
            Ok(num) if num > 0 && num <= lines.len() => num,
            _ => {
                println!("\n{}", "! Invalid ID.".red().bold());
                println!("");
                return Ok(());
            }
        };

        lines.remove(id - 1);
        let mut final_content = lines.join("\n");
        if !final_content.is_empty() {
            final_content.push('\n');
        }

        fs::write(&self.file_path, final_content)?;
        println!("\n{}", format!("Note #{} deleted.", id).red());

        self.read_note()?;
        Ok(())
    }

    pub fn clear_note(&self) -> Result<(), Error> {
        if !std::path::Path::new(&self.file_path).exists() {
            println!("\n{}", "! No notes file found to clear.".red());
            println!("");
            return Ok(());
        }

        println!("");
        let confirm = prompt("Are you sure you want to delete ALL notes? (y/N): ")?;

        if confirm.to_lowercase().trim() == "y" {
            File::create(&self.file_path)?;
            println!("\n{}", "--- Notebook purged successfully. ---".red().bold());
        } else {
            println!("\n{}", "Operation cancelled.".yellow());
        }
        println!("");
        Ok(())
    }
}
