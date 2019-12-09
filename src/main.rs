extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("sup man, I let you guess the number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("type a number, stupid.");

    println!("you just guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("Too large"),
        Ordering::Equal => println!("don't look at me stupid. You're right."),
    }
}
