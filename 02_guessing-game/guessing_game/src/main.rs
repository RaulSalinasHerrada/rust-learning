use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess number");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input guess.");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Not a number!"); continue},
        };
            
                    
        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "Too smol".blue()),
            Ordering::Greater => println!("{}", "Too BIG!".red()),
            Ordering::Equal => {
                println!("{}", "Nicely done :D".green());
                break;}
            };
    }

}
