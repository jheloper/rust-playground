pub fn example_match() {
    
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Washington);

    println!("penny coin value is {}", value_in_cents(penny));
    println!("nickel coin value is {}", value_in_cents(nickel));
    println!("dime coin value is {}", value_in_cents(dime));
    println!("quarter coin value is {}", value_in_cents(quarter));
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
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}