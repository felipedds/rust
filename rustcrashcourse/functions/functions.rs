// Function do_stuff
fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {} {}",qty * oz, qty, oz);
    return qty * oz;
}

// Function that calculate area
fn area(x: i32, y: i32) -> i32 {
    return x * y
}

// Function main
fn main() {
    do_stuff(33.1, 12.2);

    let area = area(16, 19);
    println!("Area is: {}", area);
}