use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // default rust use i32
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Guess number:");
        let mut guess = String::new(); // :: is an associated function of a type
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // pattern matching
            Err(_) => continue, // _ is a catchall value
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
