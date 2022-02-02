fn main() {
    // for loop
    for i in (0..10).step_by(1) {
        println!("{}", i);
    };

    // iterate over vectored elements
    let nums = vec![1, 2, 3, 4, 5];
    for num in nums {
        println!("{}", num);
    }

    // loop
    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        i += 1;
        println!("{}", i);
    };

    // loop
    let mut i = 0;
    let _x = loop {
        if i == 10 {
            break;
        }
        i += 1;
        println!("{}", i);
    };
    
}