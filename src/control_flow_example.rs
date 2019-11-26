pub fn example_control_flow() {
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
}