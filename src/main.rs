use std::io::stdin;
use crate::types::GUESS;

mod types;

pub fn get_index_number() -> GUESS {

    rand::random_range(1..10)
}

fn main() {
    let guess = get_index_number() as GUESS;
    let mut guesser = String::new();

    stdin().read_line(&mut guesser).expect("No Input");
    let index_guess: i32 = guesser.trim().parse::<i32>().unwrap();
    if index_guess == guess {
        println!("Guess Correct");
    } else {
        println!("Incorrect Guess");
        println!("Guess was: {}", guess);
    }
}


