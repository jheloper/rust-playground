pub fn example_data_type() {
    // 변수 타입을 선언하지 않을 경우 아래 코드에서 에러 발생
    // 따라서 변수 타입을 선언
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess is: {}", guess);

    let integer: i32 = 15000;
    println!("integer is: {}", integer);

    let float: f64 = 132.414;
    println!("float is: {}", float);

    let boolean: bool = false;
    println!("boolean is: {}", boolean);

    let character: char = 'A';
    println!("character is: {}", character);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (tup_x, tup_y, tup_z) = tup;
    println!(
        "The value of tup_x is: {}, tup_y is: {}, tup_z is: {}",
        tup_x, tup_y, tup_z
    );
    println!(
        "The value of tup.0 is: {}, tup.1 is: {}, tup.2 is: {}",
        tup.0, tup.1, tup.2
    );

    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    println!("first is: {}, second is: {}", first, second);
}