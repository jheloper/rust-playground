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
}
