#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn example_struct() {
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1 is {:?}", user1);

    let user2 = build_user(String::from("user2@example.com"), String::from("username123"));
    println!("user2 is {:?}", user2);

    // user2의 일부 값을 재사용하여 새로운 User 인스턴스 생성.
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("username456"),
        active: user2.active,
        sign_in_count: user2.sign_in_count
    };
    println!("user3 is {:?}", user3);

    // 구조체 갱신 문법(struct update syntax)으로 user2의 일부 값을 재사용.
    let user4 = User {
        email: String::from("user4@example.com"),
        username: String::from("username789"),
        ..user2
    };
    println!("user4 is {:?}", user4);

    example_tuple_struct();
}

fn build_user(email: String, username: String) -> User {
    // 변수명과 구조체의 필드명이 같다면 아래처럼 필드 초기화 축약법(field init shorthand)을 이용할 수 있음.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

// 튜플 구조체는 구조체명을 통해 의미 부여할 수 있으나 필드 타입만 정의할 수 있고 명명은 할 수 없음.
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn example_tuple_struct() {
    // 아래 black과 origin은 구조체 내의 타입이 모두 동일하지만 각각의 구조체가 고유의 타입이기 때문에 다른 타입.
    let black = Color(0, 0, 0);
    println!("black color is {:?}", black);
    
    let origin = Point(0, 0, 0);
    println!("origin point is {:?}", origin);
}