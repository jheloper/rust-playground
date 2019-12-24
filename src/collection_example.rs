pub fn example_collection() {

    let vector1: Vec<i32> = Vec::new();
    // vec! 매크로를 사용하여 초기값을 가진 Vector 생성
    let vector2 = vec![1, 2, 3, 4, 5];

    let mut vector3: Vec<i32> = Vec::new();
    vector3.push(5);
    vector3.push(6);
    vector3.push(7);
    vector3.push(8);
    println!("vector is {:?}", vector3);

    let third1: &i32 = &vector2[2];
    let third2: Option<&i32> = vector2.get(2);
    println!("vector third elements is {}", third1);
    println!("vector third elements is {:?}", third2);
}
