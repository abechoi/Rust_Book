<div align="center">
<h1>Programming a Guessing Game</h1>
<h2>Chapter 2 of the Rust Book</h2>
<p>By Abe Choi</p>
</div>

<p align="center">
This is a summary of the guessing game in Chapter 2, an explanation on the 5 most important lines.
</p>

1.  [Generate a Random Number](#Generate-a-Random-Number)
2.  [Create a String](#Create-a-String)
3.  [Read Line into String](#Read-Line-into-String)
4.  [Parse String into Number](#Parse-String-into-Number)
5.  [Compare String and Input](#Compare-String-and-Input)


## Generate a Random Number

To use the rand crate, add it in the dependencies in `Cargo.toml` and run `cargo build`. Import crate with `use rand::Rng;`. Use `gen_range(1..=6)` to set a range for the random number, from 1, to and including 6.

```
// create a random number as a u8, from 1, to and including 6
let secret_number: u8 = rand::thread_rng().gen_range(1..=6); 
```
 
## Create a String

Create a new String to store input into. It has to be mutable to bind to the new incoming input.

```
// create a new String
let mut input = String::new();
```

## Read Line into String

Import IO with `use std::io;`. Use `read_line(&mut input)` to read input into a String's address. `read_line()` returns a `Result` type, use `expect()` to handle any error `Result` may return.

```
// read line and store input into a String
io::stdin().read_line(&mut input).expect("Invalid input!");
```

## Parse String into Number

Parse String into a shadowing variable, trim to remove whitespaces and newline. `parse()` returns a `Result` type, which we will use match to respond accordingly. If there are no errors, `Ok(num)` returns the parsed number, else `Err(_)` returns `continue` to continue no matter the error.

```
// trims and parses String into a shadowing variable
let input = match input.trim().parse::<u8>() {
    Ok(num) => num,
    Err(_) => continue,
};
```

## Compare String and Input

Import `Ordering` with `use std::cmp::Ordering;`. Use `cmp()` to compare number with the random number's address. Use `match` function to create an output for anything the compare function may return. If match returns `Equals`, break out of loop.

```
// compare number and random number, output results accordingly using match function.
match input.cmp(&secret_number) {
    Ordering::Less => println!("Too low!"),
    Ordering::Greater => println!("Too high!"),
    Ordering::Equals => {
        println!("You win!");
        break;
    }
}
```
