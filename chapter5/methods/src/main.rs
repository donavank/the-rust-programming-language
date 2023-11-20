#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Here we use an immutable reference to self, but methods can also take ownership of self and borrow mutably
    fn area(&self) -> u32 { 
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height ||
        self.width > other_rect.height && self.height > other_rect.height
    }
}

// A struct can have multiple impl blocks as shown here. There's no reason for this in this example, but
// a case where multiple impl blocks are useful will be explored with generic types and traits in Chapter 10
impl Rectangle {
    // We can define associated functions that don't have self as an argument (and therefore aren't methods). These
    // functions are most commonly used as constructors like the square constructor below. An example of one we've already
    // used is the String::from() function. These constructors are also often called new()
    fn square(size: u32) -> Self {
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // As noted in the book, method calls are syntactic sugar for function calls. 
    // Rust does not have an equivalent to struct->method(); struct.method() will automatically
    // dereference struct if it must

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // Can call methods for a struct in an "instance" way like in OOP languages.
        // This is known as method syntax
    );

    println!(
        "The are of the rectangle is {} square pixels.",
        Rectangle::area(&rect1) // Can call methods for a struct in a "static" way by using :: and passing a reference to the object
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    let size = 50;
    let square1 = Rectangle::square(size);
    println!("A square with size {} has area {}",
        size,
        square1.area()
    );
}