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

    // 가변성 변수와 섀도잉의 차이. 가변성 변수를 사용하면 기존 타입과 다른 타입의 값이 들어올 경우
    // 에러가 발생한다.
    // 섀도잉을 사용하면 타입이 다른 값이라도 할당할 수 있다.
    let space = "    ";
    let space = space.len();
    println!("space length is: {}", space);

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
    println!("The value of tup_x is: {}, tup_y is: {}, tup_z is: {}", tup_x, tup_y, tup_z);
    println!("The value of tup.0 is: {}, tup.1 is: {}, tup.2 is: {}", tup.0, tup.1, tup.2);

    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    println!("first is: {}, second is: {}", first, second);

    another_function(10, 15);
}

fn another_function(x: i32, y: i32) {
    println!("This is another function! argument x is: {}, y is: {}", x, y);
}
