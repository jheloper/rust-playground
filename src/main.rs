mod mod_example;
mod ownership_example;
mod reference_example;
mod slice_example;

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

    another_function(10, 15);
    block_expression();

    let function_result = return_ten();
    println!("function result is: {}", function_result);

    let if_number = 5;

    if if_number < 1 {
        println!("less than 1.");
    } else if if_number < 10 {
        println!("between 1 and 9.");
    } else {
        println!("equal or greater than 10.");
    }

    // 아래와 같이 표현식의 결과값을 변수에 대입할 수 있다.
    // 다만 반환하는 결과값의 타입이 다르면 컴파일 오류 발생.
    let condition = true;
    let expression_number = if condition {
        7
    } else {
        // "twelve"
        12
    };
    println!("expression number is: {}", expression_number);

    let mut loop_count = 0;
    loop {
        loop_count += 1;
        println!("loop count is: {}", loop_count);

        if loop_count == 5 {
            break;
        }
    }

    let mut while_count = 3;

    while while_count != 0 {
        println!("while count is: {}", while_count);
        while_count -= 1;
    }

    let for_example_array = [10, 20, 30, 40, 50];

    for element in for_example_array.iter() {
        println!("for loop example array element is: {}", element);
    }

    for index in 0..4 {
        println!("for loop index is: {}", index);
    }

    for index in (0..4).rev() {
        println!("for loop index is: {}", index);
    }

    // Rust의 문자열 리터럴 값은 불변.
    // 또한 컴파일 시점에서 값이 정해지지 않는 경우도 존재함.
    // 따라서 문자열 리터럴이 아닌 문자열 타입인 String을 제공함.
    let mut example_string = String::from("hello");
    example_string.push_str(", world!");
    println!("example String value is: {}", example_string);

    ownership_example::example_ownership();

    reference_example::example_reference();

    slice_example::example_slices();

    mod_example::example_module();
}

fn another_function(x: i32, y: i32) {
    println!(
        "This is another function! argument x is: {}, y is: {}",
        x, y
    );
}

fn block_expression() {
    let x = 10;

    let y = {
        let x = 5;
        x + 20
    };

    println!("The value of y is: {}", y);
}

fn return_ten() -> i32 {
    10
}