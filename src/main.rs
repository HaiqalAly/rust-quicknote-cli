use std::fs::OpenOptions;
use std::{io::Write, io::Error};
use std::env;
use chrono::{self, DateTime, Utc};

fn main() -> Result<(), Error>{
    // TODO:
    //       1. Make it actually wait for user input like in Python with interactive style.
    //          Currently it uses argument e.g cargo run -- "Remember to take a break" to work.
    //          Maybe I'll just keep it like this?
    //       2. Add color for the text
    println!("========== WELCOME TO THE QUICK NOTE APP =========");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: 'Your Notes' ");
        return Ok(());
    }

    let message = &args[1];
    let time: DateTime<Utc> = Utc::now();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")?;

    writeln!(file, "{} {}", message, time)?;

    println!("Note saved!");

    Ok(())
}
