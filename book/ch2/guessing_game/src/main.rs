use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();
    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        println!("You guessed: {}", guess);

        // check quit or q

        guess = guess.trim().to_string();
        if guess == "q" || guess == "quit" {
            break;
        }
        // convert string to int
        let guess: u32 = match guess.parse() {
            Ok(result) => result,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

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
