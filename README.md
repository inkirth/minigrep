
# minigrep

Minigrep is a lightweight command-line search tool built in Rust. It is a simplified implementation of the classic grep utility, developed by following Chapter 12 of "The Rust Programming Language" book. This project serves as a practical demonstration of core Rust concepts, including file I/O, error handling, environment variables, and test-driven development.

The application searches for a specific text string within a given file and prints any matching lines directly to the standard output.

## Usage

To run the program, you must provide a search query and a file path as arguments. You can execute it directly using Cargo.

```bash
cargo run -- search_string target_file.txt

```

## Case-Insensitive Search

Minigrep supports case-insensitive searching controlled by environment variables. To enable this feature, set the IGNORE_CASE variable before executing the program.

```bash
IGNORE_CASE=1 cargo run -- search_string target_file.txt

```

