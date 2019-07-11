#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    let coin = Coin::Quarter(State::Alaska);    
    let coin2 = Coin::Penny;
    let coin3 = Coin::Quarter(State::Alabama);

    let mut count = 0;

    value_in_cents(&coin);
    
    let six = plus_one(Some(5));
    let none = plus_one(None);

    println!("{} - {}", six.unwrap(), none.unwrap_or(666));

    coin_counter(&coin, &mut count);
    coin_counter(&coin2, &mut count);
    coin_counter(&coin3, &mut count);

    println!("{} coins are not from Alaska", count);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<u32>) -> Option <u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn coin_counter(coin: &Coin, count: &mut i32) {
    if let Coin::Quarter(State::Alaska) = coin {
        println!("State quarter from Alaska");
    } else {
        *count += 1;
    }
}
