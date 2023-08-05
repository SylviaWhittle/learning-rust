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

    // UTF-8 Encoded text with Strings
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = String::from("initial contents");

    // Updating a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // note that push_str() takes a string slice as we don't usually
    // want to take ownership of the string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // The push method for strings just pushes a single character, as
    // strings are vectors.
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with + or format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    // Note that we move s1 here so it cannot be used any more
    // We need the & for s2 as the + operator takes a String, and a &str
    // slice reference. It does not take two Strings, or two refrences.
    // This is to save on memory.
    // Note that &s2 is actually a &String type and not a &str slice,
    // but rust can dereference &String to &str for us.
    let s3 = s1 + &s2;

    // format!
    // For longer and more complex concatenation, use format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1} - {s2} - {s3}");

    // Indexing into Strings
    // You cannot index a string like you would in Python
    let s1 = String::from("hello");
    // let h = s1[0];
    // Why not?
    // A String is a wrapper over a Vec<u8>.
    let hello = String::from("hola");
    // Here the length will be 4 which means it is 4 bytes long.
    // Each letter takes 1 byte.
    // This is not always the case though
    let hello = String::from("Здравствуйте");
    // This String takes two bytes of storage per character!
    // Therefore an index won't correlate to a valid Unicode
    // scalar value.

    // Iterating over Strings
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Hash maps
    // Type: HashMap<K, V>
    // Stores keys and values.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // All keys must have the same type as each other.

    // Accessing a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // Note that .get returns an Option<&V>; if there is no value for that key
    // in the hash map, get will return None. We handle this error by
    // calling copied to get an Option<i32> rather than an Option<&i32>,
    // then unwrap_or to set score to zero if scores doesn't have an entry
    // for that key.
    // Note: unwrap_or() returns the value, or if none, a default value specified.

    // Iterating over each key, value pair in a HashMap
    for (key, value) in &scores {
        println!("{key} : {value}")
    }

    // Note that this will print them in an arbitrary order

    // Updating hashmaps
    // Overwriting values
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores)
    // Adding only if key isn't present
    scores.entry(String::from("Yellow")).or_insert(50);
    // Updating a value based on an old value
    // This code counts how many times each word appears in some text
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // The or_insert method returns a mutable reference (&mut V) to the value
    // for the specified key.
    // We store the mutable reference in the count variable, so we have to 
    // dereference count first using the * operator to assign the value.

    // Hashing functions
    
}
