use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("Your guess: {guess}");
    println!("Secret number is: {secret_number}");
}