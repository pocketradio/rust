use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("the secret number is {secret_number}");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            // match just matches the Result returned by the parse() with the Ok and Err.
            // the result can only be one of those two so it checks which one it is.
            Ok(num) => num, // return num from this match ; guess will now store num
            Err(_) => continue, // skip loop if failed
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too smol"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win :D");
                break;
            }
        }
    }
}
