#[derive(Debug)]

enum UsState {
    Colorado,
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
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
        Some(i) => Some(i+1),
    }
}

fn main() {

    value_in_cents(Coin::Penny);

    value_in_cents(Coin::Quarter(UsState::Colorado));

    let five = plus_one(Some(5));

    println!("The variable five is {:?}", five);

    let none = plus_one(None);

    println!("The variable none is {:?}", none);

    // if let syntax

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
