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

    example_ownership();

    example_reference();
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

fn example_ownership() {
    // 소유권 예제 코드.

    // 아래의 경우 리터럴의 깊은 복사본이 만들어지기 때문에 정상적으로 출력 가능.
    let x = 5;
    let y = x;
    println!("This value of x is: {}, y is: {}", x, y);

    // 아래의 경우 이동(move)이 발생하기 때문에 s1은 더 이상 유효하지 않음.
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("This value of s1 is: {}", s1);
    println!("This value of s2 is: {}", s2);

    // 아래의 경우 clone 메서드를 이용해서 명시적으로 깊은 복사본을 만들 수 있음.
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("This value of s3 is: {}, s4 is: {}", s3, s4);

    // 아래의 경우 복사가 발생하기 때문에 함수 인자로 넘긴 후에도 사용 가능.
    let z = 5;
    makes_copy(z);
    println!("This value of z is: {}", z);
    // 아래의 경우 이동이 발생하기 때문에 함수 인자로 넘긴 후에는 사용 불가능.
    let s5 = String::from("hello");
    takes_ownership(s5);
    // println!("This value of s5 is: {}", s5);

    // 아래의 경우 s6에 소유권이 이동됐으므로 사용 가능.
    let s6 = gives_ownership();
    println!("This value of s6 is: {}", s6);

    // 아래의 경우 s7에서 s8으로 소유권이 이동됐으므로 s7은 무효.
    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    // println!("This value of s7 is: {}", s7);
    println!("This value of s8 is: {}", s8);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn example_reference() {
    // 참조자 예제 코드.

    let s1 = String::from("hello");
    // &s1를 넘겼으므로 소유권이 이동하지 않고 값을 참조할 수 있음.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let s2 = String::from("hello");
    change(&s2);
    println!("The s2 value is '{}'", s2);

    // 아래는 가변 참조자.
    let mut s3 = String::from("hello");
    change_mutable(&mut s3);
    println!("The s3 value is '{}'", s3);

    // 가변 참조자의 빌림은 특정 스코프 내에서 하나만 허용된다.
    let mut s4 = String::from("hello");
    let r1 = &mut s4;
    // let r2 = &mut s4;
    println!("The r1 value is '{}'", r1);
    // println!("The r2 value is '{}'", r2);

    // 아래와 같이 가변 참조자를 다른 스코프 내에서 빌리는 것은 가능하다.
    let mut s5 = String::from("hello");
    {
        let r3 = &mut s5;
        println!("The r3 value is '{}'", r3);
    }
    let r4 = &mut s5;
    println!("The r4 value is '{}'", r4);

    // 아래와 같이 불변 참조자를 가지고 있는 경우 새로 가변 참조자를 가질 수 없다.
    let mut s6 = String::from("hello");
    let r5 = &s6;
    let r6 = &s6;
    // let r7 = &mut s6;
    println!("The r5 value is '{}'", r5);
    println!("The r6 value is '{}'", r6);
    // println!("The r7 value is '{}'", r7);
}

fn calculate_length(s: &String) -> usize {
    // 인자로 &String을 선언하며 스트링 참조자를 받으므로 s는 소유권을 가지지 않음. 이것을 '빌림'이라고 표현.
    s.len()
}

fn change(some_string: &String) {
    // 아래처럼 참조자의 값을 변경할 수 없다.
    // some_string.push_str(", world!");
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world!");
}