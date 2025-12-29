fn main() {
    let config_max = Some(3u8);
    // Match
    match config_max {
        Some(max) => println!("The maximum number is {}", max),
        _ => ()
    };

    // Igual pero con if
    if let Some(max) = config_max {
        println!("The maximum number is {}", max);
    }

    // Con match
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    println!("count is {}", count);

    // If let con else
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("count is {}", count);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    #[allow(dead_code)]
    Alaska,
}

enum Coin {
    #[allow(dead_code)]
    Penny,
    #[allow(dead_code)]
    Nickel,
    #[allow(dead_code)]
    Dime,
    Quarter(UsState)
}