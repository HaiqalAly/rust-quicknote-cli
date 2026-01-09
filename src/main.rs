use std::fs::OpenOptions;
use std::io::Write;
use std::env;

fn main() {
    // TODO: 1. Add timestamps.
    //       2. Make it actually wait for user input like in Python with interactive style.
    //          Currently it uses argument e.g cargo run -- "Remember to take a break" to work.
    //          Maybe I'll just keep it like this?
    println!("========== WELCOME TO THE QUICK NOTE APP =========");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: note <your message> (WIP)");
        return;
    }

    let message = &args[1];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")
        .expect("Could not open file.");

    writeln!(file, "{}", message).expect("Could not write to file.");

    println!("Note saved!");
}
