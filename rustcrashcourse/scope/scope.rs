fn main() {
    let x = 5;
    {
        let x = 99;
        println!("{}", x); // print 99
    }
    println!("{}", x); // print 5

    let y = 3; // y is mutable
    let y = y; // y is now immutable
    print!("{}", y); // print 3
}