# Crabby

A simple Rust application for managing text-based notes. 
This tool allows users to append notes to a specific file with automatic indexing and basic CRUD (Create, Read, Delete) functionality.

## Feature
- Use the default diary.txt or specify your own file. Crabby automatically handles .txt extensions if you forget them.
- Every note is saved with a precision timestamp using the chrono crate.
- Get insights into your writing habits with word counts and density reports.
- Quickly find specific notes using keyword search.
- Functions to take, read, and delete notes by their ID.

## Usage

### Prerequisites
Rust and Cargo installed on your system.

### Running the Application
```bash
# Clone the repository
git clone https://github.com/Isvane/crabby.git

# Navigate to the project directory
cd crabby

# Run the program
cargo run
```

## Commands

Once the application is running, you can use the following commands at the >> prompt:
- `add`: Enter note-taking mode (type 'quit' to return to main menu).
- `list`: Display all notes in the current file with their IDs.
- `search`: Search for specific keywords within your notes.
- `stats`: View the "System Analytics Report" (Word count, density, etc.).
- `delete`: Prompts for a Note ID to remove a specific entry.
- `clear`: Clears the terminal screen.
- `purge`: Permanently deletes all notes in the current file.
- `quit`: Safely exits the application.
