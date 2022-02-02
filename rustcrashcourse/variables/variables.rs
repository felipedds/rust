// Function missiles
fn missiles() {
    let mut missiles = 8;
    let ready: i32 = 2;
    println!("Firing {} of my {} missiles..", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles)
}

// Function main
fn main() {
    let (bunnies, carrots) = (8, 50);
    const COLOR_RED: f64 = 3.3; // constante variable

    println!("{}, {}", bunnies, carrots);
    println!("{}", COLOR_RED);

    missiles();
}