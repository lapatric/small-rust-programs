fn main() {
    println!("This is referenc.rs");

    let s = String::from("hello");
    // s.push_str(", world!");
    // change(&s); 

    let mut a = String::from("hello");
    change_mut(&mut a);
    println!("{a}");
}

// because variables are immutable by default, so are their references 
// This function doesn't compile!
// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }

// This compiles!
fn change_mut(some_string: &mut String) {
    some_string.push_str(", world!");
}