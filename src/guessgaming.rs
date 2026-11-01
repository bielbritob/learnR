use std::io;

pub fn main() {
    println!("-[ Guess the number! ]-");
    println!(" Pls guess one nunber btw 1 ... 7");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("\nYour guess: {guess}");

}