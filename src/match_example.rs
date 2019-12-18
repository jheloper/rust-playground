pub fn example_match() {
    
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;

    println!("penny coin value is {}", value_in_cents(penny));
    println!("nickel coin value is {}", value_in_cents(nickel));
    println!("dime coin value is {}", value_in_cents(dime));
    println!("quarter coin value is {}", value_in_cents(quarter));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}