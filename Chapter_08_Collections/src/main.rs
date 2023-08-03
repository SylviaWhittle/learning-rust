fn main() {
    // Collections contain multiple values and are stored on the heap
    // so they can shrink and grow.

    // Vectors allow storing variable number of values
    // String are collections of characters
    // Hash maps associate keys with values. (Like a dictionary in Python)

    // Vectors
    // Here is how to create a vector with a type hint to tell it what it
    // is storing
    let v: Vec<i32> = Vec::new();

    // This macro allows us to initialise a vector without all that
    // boilerplate
    let v = vec![1, 2, 3];

    // Adding values to vectors
    // If we don't add mut to a variable then it cannot be added to
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading values from a vector
    // There are two ways to read values, indexing and the get() method

    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("third element is {third}"),
        None => println!("there is no third element"),
    }

    // Note that we cannot do this:

    // let mut v = vec![1, 2, 3, 4 ,5];
    // let first = &v[0];
    // v.push(6);

    // this is because we have a mutable reference and an immutable reference.
    // this could cause the location of the memory in RAM to be moved
    // if the size of the vector changes, and so is unsafe

    // Iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // we can also make mutable references to each element in a mutable
    // vector in order to make changes to the elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // To change the value, we have to use the * dereference operator
        // to get the value in i before we use the += operator.
        *i += 50;
    }
    println!("{:?}", v);

    // Using an enum to store multiple types
    // Vectors can only store values of the same type. This can be inconvenient.
    // We can use enums to get around this
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping a vector drops its elements
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed along with its elements
}
