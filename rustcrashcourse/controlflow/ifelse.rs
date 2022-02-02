fn main() {

    let age: i32 = 21;

    // if, else
    let num: i32 = 5;
    if num == 5 {
        println!("five");
    } else if num == 4 {
        println!("four");
    } else {
        println!("other");
    };

    // if, else with output capture
    let new_enough = if age < 21 {
        true
    } else {
        false
    };
    println!("{}", new_enough);

    // if, else compressed version 
    let old_enough = if age < 21 { true } else { false };
    println!("{}", old_enough);
}