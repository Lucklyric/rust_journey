#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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

fn main() {
    println!("Hello, world!");

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(1);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 =>  remove_fancy_hat(),
        other => move_player(other),
    }

    let config_max = Some(3u8);
    println!("{:?}", config_max);

    match config_max {
        Some(max) => println!("{}", max),
        None => println!("No max"),
    }

    if let Some(max) = config_max {
        println!("{}", max);
    } else {
        println!("No max");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("match None");
            None
        }
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("add_fancy_hat");
}
fn remove_fancy_hat() {
    println!("remove_fancy_hat");
}
fn move_player(num_spaces: u8) {
    println!("move_player {}", num_spaces);
}
