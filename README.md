# Crabby

A simple Rust application for managing text-based notes. 
This tool allows users to append notes to a specific file with automatic indexing and basic CRUD (Create, Read, Delete) functionality.

## Feature
- Choose or create any text file to serve as your note repository.
- Each note is automatically assigned an ID based on the current line count.
- Notes are appended to the local filesystem.
- Functions to take, read, and delete notes by their unique ID.

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
