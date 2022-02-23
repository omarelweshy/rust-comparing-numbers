use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    // Generate a random number
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Loop to allow gussing multiple times until you guess the right number
    loop {
        println!("Please guess a number");
    
        // Create new instance of String
        let mut guess = String::new();

        // Read the guess from user 
        io::stdin()
            .read_line(&mut guess)
            .expect("Falid");

        // SHADOW, shadow the guess to me unsigned integer and tirm it and parse
        // it from string to integer.
        // Also check if the input is number or string
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guess is: {}", guess);
    
        // Comparing the secret number with the guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            } 
        }
    }
} 