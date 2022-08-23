## Basic commands

```bash
# Start new rust project
cargo new hello_cargo

# compile an individual rust file main.rs
rustc main.rs 

# Compile a rust project (using cargo). 
cargo build
./target/debug/hello_cargo # run executable

# Compile and run rust project (short version of the above)
cargo run

# Check for compilation errors without compiling (fast, good practice)
cargo check

# Build for release
cargo build —release
```