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

## The Cargo.lock file
Keeps track of all dependency (crate) versions and ensures reproducibility. This file is created the first time you run `cargo build`. 

To update your crates you can call `cargo update`. Cargo will then update all crates while ensuring your specifications in `Cargo.toml` are met.

Because `Cargo.lock` is important for reproducible builds, it is common to include it in VCS.
