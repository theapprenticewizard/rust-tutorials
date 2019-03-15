#[derive(Debug)]
struct Rectangle {
    height: u32, //source control optimization!
    width: u32, // structs can end with ',' 
}

impl Rectangle {

    fn print(&self) {
        println!("{:?}", self);
    }

    // method... works like a method in python 
    // (passing a reference of self!)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn resize(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    } 

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.height < self.height && other.width < self.width
    }

    // rust equiv. of a constructor... but is more like a static factory
    // which is a good thing (new is bad in Java anyways) :)
    fn square(side_length: u32) -> Rectangle {
        Rectangle { 
            height: side_length, 
            width: side_length,
        }
    }
}

// multiple impl blocks are legal, this is for traits
impl Rectangle {
  fn print_n_times(&self, n: u32) {
        for _x in 1..n+1 {
            println!("{}: {:?}", _x, self);
        }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 20 };
    let mut square = Rectangle::square(20);

    let n = 5;
    println!("printing {} times!\n", n);
    square.print_n_times(n);
    println!("----");

    println!("resizing the square!");
    square.resize(10, 15);

    square.print();
    rect.print();
    println!("rect: {:?} has area: {}", rect, rect.area());
    println!("square: {:?} has area: {}", square, square.area());
    println!("square can hold rect: {}", square.can_hold(&rect));
    println!("rect can hold square: {}", rect.can_hold(&square));
}