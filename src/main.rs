use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    // create a variable with a random u8
    let secret_number: u8 = rand::thread_rng().gen_range(1..=6);

    loop {

        println!("Enter a guess:");

        // create a mutable variable for the user input as a string
        let mut input: String = String::new();

        // read user input and handle erroneous inputs
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid number!");

        // parse input type from a String to u8, trim whitespaces and newlines, and match the enum 
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Secret Number is {secret_number}");

        println!("You guessed {input}");

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
