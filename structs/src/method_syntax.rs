#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associate functions with the Rectangle struct
//
impl Rectangle {
    // A method of Rectangle (because it takes self)
    // &self here is actually short for self: &Self
    // where it passes a borrow of itself into the
    // function. This is like python.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // An associated function of Rectangle
    // (because it doesn't take self)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    // We say that this function is namespaced by the Rectangle
    // struct using ::
    let square = Rectangle::square(3);

    println!("Square: {:?}", square);
}
