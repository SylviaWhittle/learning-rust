// Define a struct and allow
// it to be printed by the debugger
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Create an immutable rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Debug print the rectangle
    println!("The rectangle is {:?}", rect1);

    // Calculate the area of the rectangle by passing in
    // the rectangle struct to area() which takes rectangle
    // structs
    // Only borrow the rectangle as we may want to use it afterwards.
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
