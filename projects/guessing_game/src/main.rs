use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess!");

        let mut guess= String::new(); // variables are immutable by default. With mut we make it mutable

        io::stdin()
        .read_line(&mut guess) // passing a reference of the variable. References are immutable by default. That's why mut is added.
        .expect("Failed to read line!");

        let guess: u32 = match guess
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

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
