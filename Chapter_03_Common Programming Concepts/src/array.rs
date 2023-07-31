use std::io;

fn main() {
    // arrays are all one type and fixed length
    let a = [1, 2, 3, 4, 5, 6];
    println!("a: {:?}", a);
    // can repeat elements:
    let a = [3; 5];
    println!("a: {:?}", a);
    println!("first element of a: {}", a[0]);

    println!("enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index.trim().parse().expect("index was not a valid usize");

    let element = a[index];

    println!("the value at index {index} is {element}");
}
