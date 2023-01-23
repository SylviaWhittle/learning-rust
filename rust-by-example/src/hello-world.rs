// This is a comment
// Comments are ignored by the compiler

fn main() {
    // Print text to console
    println!("hello world");

    /*
    block comments can be added like this
    */

    /// Doc comments are parsed into html library documentation
    /// generate library docs for the following item
    // //! Generate library docs for the enclosing item

    // Printing is handled by a series of macros defined in std::fmt

    // In general, the {} will automatically be replaced with any args. These will be stringified.
    println!("{} days", 31);

    // positional args can be used. specifying an integer inside {}
    // determines which additional argument will be replaced. args start
    // at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // different formatting can be invoked by specifying the format character after ":"
    println!("base 10:                    {}", 12345);
    println!("base 2 (binary):            {:b}", 12345);

    // you can right-justify text with a specified width. this will
    // output "      1".
    println!("{number:>5}", number = 1);

    // you can pad numbers with extra zeroes and left-adjust by flipping the sign.
    // this will output 10000
    println!("{number:0<5}", number = 1);

    // you can also capture arguments from a surrounding variable.
    // this will output "      1"
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // std::fmt contains many traits which govern the display of text
    // the base form of two important ones are:
    // fmt::Debug   - uses the {:?} marker. Format text for debugging purposes.
    // fmt::Display - uses the {} marker. format text in a more elegant, user friendly fashion.

    // fmt:Display trait automatically implements the ToString trait which allows us to convert the type to String.

    // DEBUG

    // all types that want ot use std::fmt formatting traits require an implementation
    // to be printable. automatic implementations are only provided for types such as in the std library.
    // All others must be manually implemented somehow.

    // The fmt::Debug trait makes this very straightforward. All types can derive (automatically create) the
    // fmt::Debug implementation. This is not true for the fmt::Display which must be manually implemented.

    //  This struct cannot be printed either with fmg::Display or fmt::Debug
    struct UnPrintable(i32);

    // The 'derive' attribute automatically creates the implementation
    // required to make this struct printable with fmt::Debug.
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // all std library types are automatically printable with {:?} too

    // derive the fmt::Debug implementation for structure. structure is a structure
    // which contains a single i32.
    #[derive(Debug)]
    struct Structure(i32);

    // put a structure inside of the structre Deep. make it printable
    #[derive(Debug)]
    struct Deep(Structure);

    fn put_structure_in_deep() {
        // printing with {:?} is similar to with {}
        println!("{:?} months in a year", 12);
        println!(
            "{1:?} {0:?} is the {name:?} name",
            "john",
            "doe",
            name = "names"
        );

        // Structure is now printable
        println!("now {:?} will print!", Structure(3));

        // the problem with Derive is that there is no control over how the results look.
        // what if we want this to just show a 7?
        println!("now {:?} will print!", Deep(Structure(7)));
    }
    put_structure_in_deep();

    // So fmt::Debug makes this printable but sacrifices elegance. Rust also provides "pretty printing"
    // with {:#?}

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    fn person() {
        let name = "john";
        let age = 27;
        let john = Person { name, age };

        // pretty print
        println!("{:#?}", john);
    }
    person();
}
