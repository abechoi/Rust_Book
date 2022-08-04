// Program: First Word
// Create a function that returns the first word of a string.

fn main() {
    let s = "hello world";

    let word = first_word(&s);

    println!("first word = {word}");
}

fn first_word(s: &str) -> &str {
    // converts String into bytes
    let bytes = s.as_bytes();

    // iter() creates an iterator over array of bytes
    // enumerate() returns tuple with a pattern
    for (i, &item) in bytes.iter().enumerate() {

        // compares value in bytes to a space in bytes
        if item == b' ' {

            // return reference of s with a range from the beginning to the index where the space was found
            return &s[..i];
        }
    }
    // return the reference of s
    &s[..]
}