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
    }
}

fn main() {
    
    let rec = Rectangle::new(10, 10);

    rec.print_squares();

    println!("\n\n{:?}", rec);
}

