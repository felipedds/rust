fn main() {
    // match statement (used like switch)
    let age: i32 = 33;
    match age {
        21 => println!("age is 21."),
        22 => println!("age is 22."),
        23 | 24 => println!("age is 23 or 24."),
        25..=58 => println!("age is between 25 and 58."),
        _ => println!("age is something else.")
    }
}