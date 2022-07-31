use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    // create a random number
    let secret_number: u8 = rand::thread_rng().gen_range(1..=6);

    loop {

        println!("Enter a guess:");

        // create a mutable string
        let mut input: String = String::new();

        // read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid number!");

        // parse input
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Secret Number is {secret_number}");

        println!("You guessed {input}");

        // compare input and string
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
