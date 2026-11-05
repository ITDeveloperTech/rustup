#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
#[allow(unused)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(unused)]
fn main() {
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    let coin = UsCoin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents_us(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    //match with value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    //match without value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    //similar match without value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_us(coin: UsCoin) -> u8 {
    match coin {
        UsCoin::Penny => 1,
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
#[allow(unused)]
fn move_player(num_spaces: u8) {}
fn reroll() {}