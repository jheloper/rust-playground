pub fn example_match() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Washington);

    println!("penny coin value is {}", value_in_cents(penny));
    println!("nickel coin value is {}", value_in_cents(nickel));
    println!("dime coin value is {}", value_in_cents(dime));
    println!("quarter coin value is {}", value_in_cents(quarter));

    let number_five = Some(5);
    let number_six = plus_one(number_five);
    let none = plus_one(None);

    println!("number five is {:?}", number_five);
    println!("number six is {:?}", number_six);
    println!("none is {:?}", none);

    let number = 7;
    match number {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let 문법 예제
    let some_8_value = Some(8);
    if let Some(3) = some_8_value {
        println!("three!");
    }
    if let Some(8) = some_8_value {
        println!("eight!");
    }

    // if let else 문법 예제
    let mut count = 5;
    if let 10 = count {
        println!("ten!");
    } else {
        count += 1;
    }
    println!("count is {}!", count);
}

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Washington,
    Ohio,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
