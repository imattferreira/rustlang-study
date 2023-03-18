/**
 * cargo update -> update cargo.lock
 * cargo doc --open -> generate docs about deps
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // macro
    println!("Guess the number!");
    let secret_number: u16 = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            // ARM is a pattern to match against and the code that should run if
            // the value given to match fits that arm's pattern
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
