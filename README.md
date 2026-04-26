# Rust Logs
A completed playground project to practice **Error Handling**, **File I/O**, and **Pattern Matching** in Rust. This was built as part of my journey through the Stephen Grider Rust course on Udemy.

## Project Overview
This repository serves as a showcase of progressing from basic Rust syntax to idiomatic, professional-level code. It demonstrates how to safely handle file operations and process text while keeping error handling clean and readable.

### Key Concepts Applied
- **Log Parsing**: Reading from a text file (`logs.txt`) and filtering specific lines to automatically generate `errors.txt` and `warnings.txt` using `std::fs`.
- **Result Handling & The `?` Operator**: Implementing clean error propagation using `Result<T, E>` and the `?` operator, completely avoiding verbose, deeply nested `match` statements.
- **Pattern Matching**: Using `match` blocks to handle different data outcomes gracefully and safely.
- **Validation Logic**: Simple tools like email validation and safe division.
- **WSL Dev**: Built and tested entirely within **Ubuntu on WSL2**.

## How to run
Since this is a standard Cargo project, ensure you have a `logs.txt` file in the root directory. Then, just use the terminal:

```bash
# Clone it
git clone [https://github.com/iHuGi/rust_logs.git](https://github.com/iHuGi/rust_logs.git)

# Run it
cargo run -q