use std::io::Error;
use colored::Colorize;


pub fn read_note(path: &str) -> Result<(), Error> {
    if !std::path::Path::new(path).exists() {
        println!("{}", "File not found! Please add a note first.".red().bold());
        return Ok(());
    }

    let content = std::fs::read_to_string(path)?;
    println!("\n{}", "Your notes:".bold());

    if content.is_empty() {
        println!("{}", "Notes empty".red().bold());
    } else {
        for line in content.lines() {
            if let Some((message, time)) = line.rsplit_once('[') {
                let time = time.trim_end_matches(']');
                println!("{} {} {}", ">".blue(), format!("[{}]", time).dimmed(), message.trim());
            } else {
                println!("{} {}", ">".blue(), line);
            }
        }
    }

    Ok(())
}