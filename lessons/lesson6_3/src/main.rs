#[derive(Debug)]
#[allow(unused)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum of configured to be {max}"),
        _ => (),
    }

    //similar upper if we can't want use [_ => ()] variables
    if let Some(max) = config_max {
        println!("The maximum of configured to be {max}");
    };

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }
    println!("Count {count}");

    //Similar to upper
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }
    println!("Count {count}");
}
