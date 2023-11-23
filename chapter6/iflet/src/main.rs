enum Coin {
    Quarter(String),
    Dime,
    Nickel,
    Penny,
}

fn main() {
    let mut coin: Coin = Coin::Quarter(String::from("Pennsylvania"));
    count_coin(&coin);
    coin = Coin::Dime;
    count_coin(&coin);
    coin = Coin::Nickel;
    count_coin(&coin);
    coin = Coin::Penny;
    count_coin(&coin);

}

fn count_coin(coin: &Coin) {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("Count: {count}");
}

// Makes a string to separate lines of text, 
/// returning a default if the provided string is blank
fn make_separator(user_str: &str) -> &str {
    if user_str == "" {
        let default = "=".repeat(10);
        &default
    } else {
        user_str
    }
}