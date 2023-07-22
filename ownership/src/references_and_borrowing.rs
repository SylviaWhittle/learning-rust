fn main() {
    let s1 = String::from("hello");

    // we can pass refrences to variables to functions and then return them
    let len = borrow_variable(&s1);

    // Mutable references

    let mut s1 = String::from("hello");

    // we can allow editing of borrowed variables using mutable references
    edit_borrowed_variable(&mut s1);

    println!("{}", s1);

    // we can have two mutable references but they have to be in different scopes
    // if they are in different scopes, then even when the program is multithreaded
    // it's impossible for the two references to exist at the same time.

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope so we can make a new reference

    let r2 = &mut s;

    // combining mutable and non-mutable references behave similarly
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; NOT ALLOWED


}

fn borrow_variable(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // here s goes out of scope, but as it is a reference, and so does not have ownership of
  // the data it refers to, the data is not dropped

// we cannot modify a borrowed variable

/*

fn edit_borrowed_variable( s: &String) -> usize {
    s.push_str(", world");
}

*/

// this fails as we are trying to edit a non-mutable reference

// if we want to edit a reference's value, we can take a mutable reference
fn edit_borrowed_variable(s: &mut String) {
    s.push_str(", world");
}

// we can only have one mutable reference at a time in a scope, this is to prevent data races when
// multithreading

/*

fn two_mutable_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println("{} {}", r1, r2);
}

*/

// Dangling references get caught at compile time

/*

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // s goes out of scope and so the return value points to nothing.

*/


