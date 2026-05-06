use colored::Colorize;
use std::io::Error;
use std::path::PathBuf;

mod notes;
mod utils;

use notes::{clear_note, delete_note, read_note, search_note, show_stats, take_note};
use utils::prompt;

fn main() -> Result<(), Error> {
    println!(
        "{}",
        "==== CRABBY (v0.1.0) ====".truecolor(255, 69, 0).bold()
    );

    let path_input = utils::prompt("Select file (default: diary.txt): ")?;
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
        "list, add, search, delete, stats, purge, clear, quit".yellow()
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
            "purge" => clear_note(&final_path)?,
            _ => println!(
                "{} {}. Try: {}",
                "! Unknown command".red(),
                cmd.white().bold(),
                "list, add, delete, stats, purge, clear, quit".yellow()
            ),
        }
    }
}
