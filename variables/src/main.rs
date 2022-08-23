
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mutable and shadowing
    let mut x = 5;
    let x = 6;
    let mut x = 7;
    println!("x is {x}");
    let mut spaces = "   ";
    let spaces = spaces.len();
    println!("{}", THREE_HOURS_IN_SECONDS);

    // parsing
    let guess: u32 = "42 ".trim().parse().expect("Not a number!");
    println!("guess = {}", guess);

    // visual seperators 
    let x = 1_000;
    println!("x is {}", x);

    // type suffix
    let x: u128 = 1000u128;
    let x = 1000u128;
    let mut x = 1000u128;
    // x = 1000u64; // compiler error

    // more on types
    let x: u16 = 0xff;
    println!("{x}");

    // floats 
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{y}");

    // boolean type
    let t = true;
    let f : bool = false;
    println!("{f}");

    // chars
    let c = 'z';
    let c : char = 'z';

    // Compound Types: Tuple 
    let tuple = (39, 29, 20);
    let (x, y, z) = tuple;
    println!("({x}, {y}, {z})");
    let tuple: (u8, u16, u32) = (3, 55, 209);
    let second_number = tuple.1;
    println!("The second number in tuple is: {second_number}");
    println!("{:?}", tuple); 

    // empty tuple, aka "unit"
    let x: () = (); 

    // arrays are fixed length, use vectors for variable length
    let some_days = ["Mon", "Tue", "Wed"];
    let some_nums: [u32; 5];
    some_nums = [0, 1, 2, 3, 4];
    println!("{:?}", some_nums);
    let zeros = [0; 100]; // array with 100 0s
    println!("The second element of some_nums is: {}", some_nums[1]);

    // functions
    some_function(32, "Alice");

    // expressions vs statements
    // expressions return a value, statements don't
    let y = {
        let x = 5;
        x + 1
    };
    println!("y evaluates to {}", y);
    // This following statement returns (), i.e. no return value
    let y = {
        let x = 5;
        x + 1; // only difference is the semi-colon
    };

    // functions with return values
    println!("simple_fun returns {}", simple_fun());
    println!("simple_fun_2 returns {}", simple_fun_2(9));

    // if-else statements
    let x = (3, 5);
    let max_x;
    if (x.0 > x.1) {
        max_x = x.0;
    } else {
        max_x = x.1;
    }
    println!("The max of {:?} is {}.", x, max_x);
    let min_x = if x.0 <= x.1 {x.0} else {x.1};
    println!("The min of {:?} is {}.", x, min_x);
}

fn some_function(age: u32, name: &str) {
    println!("{} is {} years old.", name, age);
}

fn simple_fun() -> u32 {
    5
}

fn simple_fun_2(x: u32) -> u32 {
    x + 1
}