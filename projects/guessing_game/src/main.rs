use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // This line could be commented to make the game more interesting
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust has a strong, static type system.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // compares the `guess` with the `secret_number` using the `cmp` method
        // The `match` expression is used to handle the different possible results of the comparison.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
