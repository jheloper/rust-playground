pub fn example_reference() {
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

    // 아래와 같이 불변 참조자를 가지고 있는 경우 새로운 가변 참조자를 가질 수 없다.
    let mut s6 = String::from("hello");
    let r5 = &s6;
    let r6 = &s6;
    // let r7 = &mut s6;
    println!("The r5 value is '{}'", r5);
    println!("The r6 value is '{}'", r6);
    // println!("The r7 value is '{}'", r7);

    // 댕글링 참조자 예제.
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
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

// 아래 함수는 댕글링 참조자를 만들기 때문에 컴파일 오류가 발생한다.
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}