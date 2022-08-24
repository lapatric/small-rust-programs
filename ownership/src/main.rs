fn main() {
    // Deep vs Shallow copies
    let s1 = String::from("hello");
    let s2 = s1; 
    // <- at this point s1 is not valid anymore, to combat double freeing 
    // of allocated memory space on the heap once s1 & s2 are out of scope

    let s3 = s2.clone();
    // Because s3 is a deep copy of s2 (heap data copied), s2 is valid
    println!("{}, world!", s2);

    // Ownership loss to functions
    let s = String::from("hello");  // s comes into scope
    // The parameter is a String type (pointer, length, capacity)
    // s within this scope loses it's ownership of the heap slot
    takes_ownership(s);  // s's value moves into the function...
    // ... and so is no longer valid here

    let s0 = String::from("hello");
    // The parameter is a simple pointer (not String). 
    // Nothing happens to s, it maintains ownership of it heap slot
    takes_pointer(&s0);    
    println!("String s0 is still valid if passed by reference / pointer: {}", s0);
    println!("The pointer of s0 is {}", &s0);   

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn takes_pointer(some_string: &String) {
    println!("{}", some_string);
} 

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.