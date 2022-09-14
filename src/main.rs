#![allow(non_snake_case)]
use rand::{random, Rng};
use std::io;

//mod print;
//mod vars;
//mod types;
//mod strings;
//mod tuples;
mod Display;
mod arrays;

fn main() {
    //println!("Hello, world!");
    //print::run(); //running the fuction called "run()" from "print.rs"
    //vars::run();
    //types::run();
    //strings::run();
    //tuples::run();
    //arrays::run();
    //Display::run();

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Equal => println!("Too big!"),
        std::cmp::Ordering::Greater => println!("You win!"),
    }
}
