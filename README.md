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

# Build documentation from your dependencies and open in browser
cargo doc --open
```

## The Cargo.lock file
Keeps track of all dependency (crate) versions and ensures reproducibility. This file is created the first time you run `cargo build`. 

To update your crates you can call `cargo update`. Cargo will then update all crates while ensuring your specifications in `Cargo.toml` are met.

Because `Cargo.lock` is important for reproducible builds, it is common to include it in VCS.

## Ownership of variables on the heap

The ownership of a variable on the heap follows the same pattern every time: Assigning a value to another variable "moves it", thus invalidating the previous instance. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another (returned) variable. 

Source: [Rust documentation](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

## References and Borrowing

To borrow a variable `s` without taking ownership we can use its poiner `&s`. For an immutable variable, we cannot change its value by using it's pointer, i.e. the following code throws a compiler error.

```Rust
let s = String::from("hello");
change(&s); 

fn change(some_string: &String) {
    some_string.push_str(", world!");
}
```

This would only be possible if `s` was mutable (and hence also it's pointer permitted mutability), i.e. `let mut s = String::from("hello"); change(&mut s);`. Naturally the function signature would need to be adapted too.

The following scenarios are not permitted by the compiler:
- Two mutable references to the same variable. [Prevents data races]
- Simultaneous immutable and mutable references to the same (mutable) variable within the same scope, bar [Non-Lexical Lifetimes](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes). [Prevents unexpected behaviour]
- Return or maintain a reference to a variable that goes out of scope, a so-called dangling pointer. [Prevents pointers to freed-up heap space]


