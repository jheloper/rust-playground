pub fn example_collection() {
    let vector1: Vec<i32> = Vec::new();
    // vec! 매크로를 사용하여 초기값을 가진 벡터 생성
    let vector2 = vec![1, 2, 3, 4, 5];

    let mut vector3: Vec<i32> = Vec::new();
    vector3.push(5);
    vector3.push(6);
    vector3.push(7);
    vector3.push(8);
    println!("vector is {:?}", vector3);

    let third1: &i32 = &vector2[2];
    let third2: Option<&i32> = vector2.get(2);
    println!("vector third element is {}", third1);
    println!("vector third element is {:?}", third2);

    match vector2.get(2) {
        Some(third1) => println!("The third element is {}", third1),
        None => println!("There is no third element"),
    }

    // []를 통해 접근하면 존재하지 않는 요소에 접근할 경우 에러 발생.
    // 반면 벡터의 get 메서드로 존재하지 않는 요소에 접근할 경우 None을 반환함.
    // let does_not_exist1 = &vector2[100];
    let does_not_exist2 = vector2.get(100);
    // println!("vector hundred element is {}", does_not_exist1);
    println!("vector hundred element is {:?}", does_not_exist2);

    // 벡터의 push 메서드를 통해 요소를 추가할 때 벡터 전체의 복사가 발생하는데,
    // 빌림 규칙에 따라 다른 참조자가 존재하면 컴파일 타임 에러 발생.
    let mut vector4 = vec![1, 2, 3, 4, 5];
    let first = &vector4[0];
    // vector4.push(6);
    println!("vector first element is: {}", first);

    // 벡터 요소 반복처리 예제
    let vector5 = vec![100, 32, 57];
    for i in &vector5 {
        println!("{}", i);
    }

    let mut vector6 = vec![100, 32, 57];
    for i in &mut vector6 {
        *i += 50;
        println!("{}", i);
    }

    // 열거형을 정의하여 벡터 내에 여러 타입의 데이터를 담을 수 있음.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("enums in a vector: {:?}", row);

    let mut string1 = String::new();

    let data = "initial contents";
    let string2 = data.to_string();
    let string3 = "initial contents".to_string();
    println!("string2 is {}", string2);
    println!("string3 is {}", string3);

    let mut string4 = String::from("foo");
    println!("string4 is {}", string4);
    string4.push_str("bar");
    println!("string4 is {}", string4);
    let string5 = "baz";
    string4.push_str(string5);
    println!("string4 is {}", string4);

    let mut string6 = String::from("lo");
    string6.push('l');
    println!("string6 is {}", string6);

    let string7 = String::from("Hello, ");
    let string8 = String::from("World!");
    let string9 = string7 + &string8;
    println!("string9 is {}", string9);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tictactoe = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", tictactoe);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
