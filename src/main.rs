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

    let result_match = UsState::existed_in(&UsState::Alabama, 2000);
    println!("Result match is {}", result_match);

    let result_if_let = UsState::describe_state_quarter(Coin::Quarter(UsState::Alabama));
    println!("Result if-let is {:?}", result_if_let);

    let result_anticipado = UsState::describe_state(Coin::Quarter(UsState::Alabama));
    println!("Result if anticipado is {:?}", result_anticipado)
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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 2013,
        }
    }

    fn describe_state_quarter(coin: Coin) -> Option<String> {
        if let Coin::Quarter(state) = coin {
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old, for America!"))
            } else {
                Some(format!("{state:?} is relatively new!"))
            }
        } else { None }
    }

    fn describe_state(coin: Coin) -> Option<String> {
        let state = if let Coin::Quarter(state) = coin {
            state
        } else {
            return None
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is highly old!"))
        } else {
            Some(format!("{state:?} is not old!"))
        }
    }
}
