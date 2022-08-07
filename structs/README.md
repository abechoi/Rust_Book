<div align="center">
<h1>Structs</h1>
<h2>Chapter 5 of the Rust Book</h2>
<p>By Abe Choi</p>
</div>

<p align="center">
This is a summary of Chapter 5, struct and mods. In this program, there is a "shapes" module with a "Rectangle" struct. To create a Rectangle type, use a constructor that takes 2 parameters for height and width, and calculates the area. Then create and call a print_squared method, to print out squares with the given dimensions.
</p>


1.  [Overview](#Overview)
2.  [Create a Struct](#Create-a-String)
3.  [Create a Constructor](#Create-a-Constructor)
4.  [Create a Function](#Create-a-Function)


## Overview

Create a shapes mod to store structs, implementation, and associated functions inside.
```
use shapes::Rectangle;

mod shapes {

    #[derive(Debug)]
    pub struct Rectangle {
        pub height: u32,
        pub width: u32,
        pub area: u32,
    }    

    impl Rectangle {
        pub fn new(height: u32, width: u32) -> Rectangle {
            Rectangle {
                height,
                width,
                area: height * width,
            }
        }
    }
}
```
 
## Create a Struct

A struct holds attributes and its types.
```
#[derive(Debug)]
pub struct Rectangle {
    pub height: u32,
    pub width: u32,
    pub area: u32,
}
```

Create a variable of a struct:
```
let rec = Rectangle {
    height: 8,
    width: 4,
    area: 32,
}
```

## Create a Constructor

The new function can be used as a constructor for creating a struct, and can return calculations before the struct is returned to the variable assignment.

```
impl Rectangle {
    pub fn new(height: u32, width: u32) -> Rectangle {
        Rectangle {
            height,
            width,
            area: height * width,
        }
    }
}
```

Create a variable of a struct, using new():
```
let rec = Rectangle::new(8, 4);
```

# Create a Function

When creating a function within an implementation, always pass an argument of &self, a borrowed self, which is shorthand for self: &self. Variables will be called using dot notation, `self.width`.
```
pub fn print_squares(&self) {
        
    for i in 0..self.width {
        print!(" _ ");

        if self.width < 10 && i == self.width-1 {
            break;
        }
    }
    for i in 0..self.area {
        if i % self.width == 0 {
            println!();
        }
        
        print!("|_|");
    }
}
```

Call function:
```
rec.print_squares();
```