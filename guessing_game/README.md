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

To use the rand crate, first add it in the dependencies in ```Cargo.toml``` and run ```cargo build```. Import crate with ```use rand::Rng;```. Use ```.gen_range()``` to set a range for the random number.

```
# create a random number as a u8, from 1, to and including 6
let secret_number: u8 = rand::thread_rng().gen_range(1..=6); 
```
 
## Create a String

Create a new String to store input into. It has to be mutable to bind to the new incoming input.

```
# create a new String
let mut input = String::new();
```

## Read Line into String

Enter information about Chapter III.

```
# this ia a code block for chapter iii.
io::stdin().read_line().expect("");
```

## Parse String into Number

Enter information about Chapter IV.

```
# this ia a code block for chapter iv.
```

## Compare String and Input

Enter information about Chapter V.

```
# this ia a code block for chapter v.
```
