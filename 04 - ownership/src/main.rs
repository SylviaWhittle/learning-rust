fn main() {
    let a = "a";

    // the variable a refers to a string literal

    {
        let b = "b";

        // b is in scope
    }
    // b is out of scope

    // create a heap allocated string
    let mut s = String::from("hello");

    // append a string literal to the string
    s.push_str(" world");

    println!("{}", s);

    // Strings can be edited but string literals cannot.
    // this is because string literals are stack allocated and strings are heap allocated.

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); <--- does not work
    // s1 is no longer in scope and is not available. ownership has been transferred to s2
    println!("{}", s2);

    // if we want to explicitly create two unique copies, we can clone() it..

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {s1} s2: {s2}");

    // there is an exception, when copying values that are stored in the stack, they don't go out of scope:
    let x = 5;
    let y = x;
    println!("x: {x} y:{y}");

    println!("-------------");

    ownership_test();

    println!("-------------");

    returning_values();

    println!("-------------");

    let s1 = String::from("hello");

    let (s2, len) = return_multiple_values(s1);

    println!("length of {} is {}", s2, len);
}

fn ownership_test() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value is transferred into
                        // the function and is no longer valid

    // println!("{}", s); // does not work

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function but
                   // x is i32 which is copy so it's still
                   // okay to use
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope for the function only
    println!("{}", some_string);
} // here some_string goes out of scope and `drop` is called.
  // the backing memory is freed

fn makes_copy(some_integer: i32) {
    // because integers are immutable and held in the stack,
    // this function makes a copy of the variable
    println!("{}", some_integer);
} // here some_integer goes out of scope. nothing special happens

fn returning_values() {
    let s1 = gives_ownership(); // gives_ownership moves its return value to s1

    println!("{}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2 is moved into the function
                                       // which returns it into s3
    println!("{}", s3);
} // s3 goes out of scope and is dropped. s2 was moved so nothing happens, s1 goes out of
  // scope and is dropped

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // remember an expression at the end of a function returns the value
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned, leaving scope
}

fn return_multiple_values(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
