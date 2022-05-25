use std::io;
extern crate rand;
use rand::Rng;

fn main(){
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1,101);
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).
    expect("Failed to read line");
    println!("You guessed: {}", guess);
    println!("The secret number is {}", secret)
}