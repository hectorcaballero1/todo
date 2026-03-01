# todo

A minimal CLI task manager for recurring tasks, built while learning Rust.

## What is this?

More than a traditional todo list, this is a simple manager for repetitive, ongoing tasks.

The circular approach helps you rotate through tasks systematically without decision fatigue.

## Installation
```bash
cargo install --path .
```

## Usage
```bash
# Create a list
todo create courses

# Add tasks
todo courses add "Algorithms"
todo courses add "Database"
todo courses add "Statistics"

# View current state
todo courses
  1. Algorithms
> 2. Database
  3. Statistics

# Get next task (rotates forward)
todo courses next
> Statistics
```

Work in progress. Code quality and features will improve over time, hopefully.