fn main() {

    // mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // const
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is: {}", MAX_POINTS);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
}
