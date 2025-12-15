use rand::Rng; // Importing the random number generation library
use std::cmp::Ordering;
use std::io; // Importing the standard input/output library // Importing the Ordering enum for comparison results

fn main() {

        println!("Guess the number ");
        let mut rng = rand::rng();
        let secret_number = rng.random_range(1..=100);
        //let secret_number = rand::thread_rng().random_range(1..=100);
        println!("Please input your guess: ");
        
    loop {
        println!("Input your guess: ");
        // Creating a mutable String to hold user input
        //The :: syntax in the ::new line indicates that new is an associated function of the String type.
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user function");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // to handle the potential error when parsing the string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
