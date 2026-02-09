# QuickNote CLI
A lightning-fast command-line tool for capturing thoughts before they vanish. Written in Rust as a learning project.

## Project Status: Complete (Learning Milestone)
This was my first project in Rust, focused on mastering CLI basics, module system and error handling. I have achieved my primary learning goals and am moving on to more complex projects.

### Few Key Learning Outcomes
- Memory Safety: Practiced string ownership and borrowing across modules.
- Crates: Integrated chrono for time and colorize for terminal UI.
- Idiomatic Rust: Refactored from if/else to match patterns.
- STD API: Used fs::OpenOptions for file manipulation, PathBuf for path handling, and io::stdin()/stdout() for user interaction.
- String Parsing & Manipulation: Used rsplit_once() for timestamp extraction and trim() variations for input sanitization.

## How it Works
QuickNote takes your input and appends it to a file with timestamps. You can specify a custom file path or press Enter to use the default `notes.txt`.
Now includes menu to either add new notes or view existing ones.

**Usage**:
```bash
# Clone the repo
git clone https://github.com/HaiqalAly/rust-cli-quicknote.git
cd rust-cli-quicknote

# Run the program
cargo run
```

## Features
- **View Notes**: Read your saved notes directly from the terminal
- **Delete Notes**: Remove old or unwanted notes by ID
- **Interactive Menu**: Persistent loop to list, add, or delete notes continuously
- **Clear Screen**: strict 'clear' command to keep your workspace tidy
- **Custom Save Location**: Choose where to store your notes on startup
- **Interactive Mode**: Enter a persistent session to quickly add multiple notes without restarting
- **Automatic Timestamps**: Each note is prefixed with [YYYY-MM-DD HH:MM:SS]
- **Colored Output**: Visual feedback with green success messages and colored prompts
- **Persistent Storage**: Automatically creates and appends to notes.txt
- **Zero Overhead**: No database required, just plain text
- **Rust Powered**: Fast, safe, and memory-efficient

See [CHANGELOG.md](CHANGELOG.md) for project updates.

## Future Ideas (If i return to this)
**Tags & Categories**: Organize notes with labels for better filtering<br>
**Path Expansion**: Support for `~` (home directory) in file paths<br>
