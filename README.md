# QuickNote CLI
A lightning-fast command-line tool for capturing thoughts before they vanish. Written in Rust as a learning project.

## How it Works
Currently, QuickNote takes your input directly from the terminal arguments and appends it to a local notes.txt file.

**Usage**:
```bash
# Clone the repo
git clone https://github.com/HaiqalAly/rust-cli-quicknote.git
cd rust-cli-quicknote

# Run the app
cargo run -- "Pick up milk on the way home"
```

## Features
- **Persistent Storage**: Automatically creates and appends to notes.txt
- **Zero Overhead**: No database required, just plain text
- **Rust Powered**: Fast, safe, and memory-efficient

## Roadmap
[ ] **Timestamps**: Automatically prefix notes with [YYYY-MM-DD HH:MM]<br>
[ ] **Interactive Mode**: A "waiting" prompt so you can type notes without restarting the app<br>
[ ] **Custom File Paths**: Allow users to specify where their notes are saved<br>
[ ] **List/Delete**: Add commands to view or clear the notes list<br>
