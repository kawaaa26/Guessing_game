use std::io;

fn main() {
    println!("sup man, I let you guess the number.");

    println!("Input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("you just guessed {}", guess);
}
