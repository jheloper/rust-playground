pub fn example_ownership() {
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