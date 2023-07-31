fn main() {
    // The null value in other languages can cause bugs.
    // In rust we use the Option enum to handle null values.

    // The <T> notation means that the Some variant of the Option
    // enum can hold one piece of data of any type, and that each
    // concrete type that gets used in place of T makes the overall
    // Option<T> type a different type.

    // enum Option<T> {
    //     None,
    //     Some(T),
    //}

    // The type of some_number is Option<i32>
    let some_number = Some(5);
    // The type of some_char is Option<char>
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // This will not compile, since y may be a null value
    // and you cannot add an i8 to a null.
    // So rust protects us by ensuring that all
    // circumstances where values could be null,
    // HAVE to be handled properly as opposed to just
    // assuming. This helps prevent bugs. A match expression
    // might be used for this.
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5)
    // let sum = x + y
}
