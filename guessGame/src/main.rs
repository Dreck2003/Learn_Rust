extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret);

    loop {
        println!("PLease enter you guess");

        let mut word = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        let word: u32 = match word.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guess is: {}", word);

        match word.cmp(&secret) {
            Ordering::Less => println!("Is small"),
            Ordering::Greater => println!("IS big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
