use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("Please input your guess");

    loop {

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    let guess: u32 = guess.trim().parse().expect( "Please type a number");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("{}","Too small".red()),
        Ordering::Equal => {
            println!("{}","Matched winner!".green());
        break;
        },
        Ordering::Greater => println!("{}","Too big".red()),
    }
}
}
