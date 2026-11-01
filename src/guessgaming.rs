use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    println!("-[ Guess the number! ]-");
    println!(" Pls guess one nunber btw 1 ... 7");
    // rand int btw 1...7
    let secret_num = rand::thread_rng().gen_range(1..=7);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // converts string to i32 and parse it
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("\nYour guessed: {guess}, me {secret_num}");
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small! lmao"),
        Ordering::Greater => println!("Too big! hahaha"),
        Ordering::Equal => println!("You win! tsch"),
    }

}