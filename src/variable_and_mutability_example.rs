pub fn example_variable_and_mutability() {
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
}