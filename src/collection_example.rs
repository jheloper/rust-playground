use std::collections::HashMap;

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

    let string10 = String::from("hello");
    // 스트링의 인덱스를 통한 문자 접근은 불가함.
    // let h = string10[0];
    println!("string10 length is {}", string10.len());

    // 아래 문자열은 UTF-8로 인코딩되어 있어 각 글자들이 2바이트를 차지함. 따라서 len은 24를 반환.
    let string11 = String::from("Здравствуйте");
    println!("string11 length is {}", string11.len());

    // 아래 코드는 인덱스 접근 오류 발생. 왜냐하면 하나의 글자가 2바이트를 차지하는데, 첫번째 바이트에만 접근하기 때문.
    // println!("string11 index 0 to 1 is {}", &string11[0..1]);
    // 아래 코드는 정상 실행.
    println!("string11 index 0 to 4 is {}", &string11[0..4]);

    // 아래 코드는 유니코드 스칼라 값 하나씩 순회
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    // 아래 코드는 바이트 값 하나씩 순회
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }

    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);
    println!("scores1 is {:?}", scores1);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores2 is {:?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map is {:?}", map);

    // 아래 코드는 에러 발생. 해쉬맵의 키와 값으로 삽입되는 순간 소유권이 이동되기 때문
    // print!("field name is {}", field_name);
    // print!("field value is {}", field_value);

    // 해쉬맵 내의 값에 접근
    let team_name = String::from("Blue");
    let team_score = scores1.get(&team_name);
    println!("team score is {:?}", team_score);

    // 해쉬맵 내의 각각의 쌍을 순회
    for (key, value) in &scores1 {
        println!("{}: {}", key, value);
    }

    // insert 메서드로 기존 값을 갱신할 수 있음
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25);
    println!("scores3 is {:?}", scores3);

    // or_insert 메서드로 해당 키에 할당된 값이 존재하지 않을 경우에만 값을 삽입
    scores3.entry(String::from("Blue")).or_insert(10);
    scores3.entry(String::from("Yellow")).or_insert(50);
    println!("scores3 is {:?}", scores3);

    // 기존 값을 기초로 값을 갱신할 수 있음
    let text = "hello world wonderful world";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_map);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
