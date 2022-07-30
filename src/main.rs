use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    // create a mutable variable for the user input as a string
    let mut input: String = String::new();

    // create a variable with a random u8
    let secret_number: u8 = rand::thread_rng().gen_range(1..=6);
    
    println!("Enter a guess:");
    println!("Secret Number is {}", secret_number);

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid number!");

    let input = input.trim().parse::<u8>().expect("Please enter a number!");

    println!("You guessed {}", input);

    match input.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You win!")
    }

    
}
