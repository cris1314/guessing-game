extern crate rand;
use std::io;
use rand::Rng;
fn main() {

    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("your secret number is: {}",secret_number);

    println!("please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).
    	expect("Failed to read line");

    println!("Your guessed: {}",guess);
}
