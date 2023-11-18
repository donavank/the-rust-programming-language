use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Enter 'exit' to quit.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        match guess.trim().to_lowercase().cmp(&String::from("exit")) {
            Ordering::Equal => {
                println!("Exiting...");
                return;
            }
            Ordering::Greater | Ordering::Less => println!("You guessed: {guess}"),
        };

        let guess: u32 = match guess.parse() {
            // match parse result to assign guess
            Ok(num) => num,     // user entered a number
            Err(_) => continue, // user did not enter a number
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // guess < secret_number
            Ordering::Greater => println!("Too big!"), // guess > secret_number
            Ordering::Equal => {
                // guess == secret_number
                println!("You win!");
                break;
            }
        }
    }
}
