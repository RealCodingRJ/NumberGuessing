use std::io::stdin;
use crate::types::GUESS;

mod types;
mod db;

pub fn get_index_number() -> GUESS {

    rand::random_range(1..10)
}

#[tokio::main]
async fn main() {
    let guess = get_index_number() as GUESS;
    let mut guesser = String::new();

    stdin().read_line(&mut guesser).expect("No Input");
    let index_guess: i32 = guesser.trim().parse::<i32>().unwrap();
    if index_guess == guess {
        println!("Guess Correct");

        let db = db::db::get_client().await;
        let dbs = db.database("Guess");
        dbs.create_collection("GuessedNumber").await.unwrap();

        let coll: mongodb::Collection<String> = dbs.collection("guess");
        coll.insert_one(guess.to_string()).await.unwrap();

    } else {
        println!("Incorrect Guess");
        println!("Guess was: {}", guess);
    }
}


