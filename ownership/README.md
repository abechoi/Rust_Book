<div align="center">
<h1>Ownership</h1>
<h2>Chapter 4 of the Rust Book</h2>
<p>By Abe Choi</p>
</div>

<p align="center">
This is a summary of Ownership in Chapter 4.
</p>

1.  [Ownership Rules](#Ownership-Rules)
2.  [First Word Function](#First-Word-Function)



## Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, value is dropped.


 
## First Word Function

Create a function that returns the first word of a string.

```
fn first_word(s: &str) -> &str {

    // convert String into bytes
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
```