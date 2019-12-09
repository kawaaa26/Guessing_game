extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("sup man, I let you guess the number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("you just guessed {}", guess);
}
