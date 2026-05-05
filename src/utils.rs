use colored::Colorize;
use std::io::{self, Error, Write};

pub fn prompt(message: &str) -> Result<String, Error> {
    print!("{}", message.bold().cyan());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn formatted_line(index: usize, line: &str) {
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
