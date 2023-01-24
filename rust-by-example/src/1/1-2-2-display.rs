// Import via the 'use' keyword. The 'fmt' module to make it available.
use std::fmt;

// define a structure for which fmt::Display will be implemented
// this is a tuple struct named Structure that contains an i32
struct Structure(i32);

// to use the {} marker, the trait fmt::Display must be implemented
// manually for the type
impl fmt::Display for Structure {
    // this trait requires 'fmt' with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write strictly the first element into the supplied output stream: 'f'.
        // returns 'fmt::Result' which indicates whether thre operation
        // succeeded or failed. Note that 'write!' uses syntax which is very
        // similar to 'println!'.
        write!(f, "{}", self.0)
    }
}

// fmt::Display may be cleaner than fmt::Debug but htis presents a problem for the std library.
// How should ambiguous types be displayed? For example, if the std library implemented a single
// style for all Vec<T>, what style should it be? Would it be either of these two?

// Vec<path> : /:/etc:/home/username:/bin (split on :)
// Vec<number> 1,2,3 (split on ,)

// No, because there is no ideal style for all types and the std library doesn't presume to
// dictate one. fmt::Display is not imlemented for Vec<T> or for any other generic containers.
// fmt::Debug must then be used for these generic cases.

// this is not a problem though because for any new container type which is not generic,
// fmt::Display can be implemented.

#[derive(Debug)]
struct MinMax(i64, i64);

// implement 'Display' for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use self.number to refer to each positional data point
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are namable for comparison
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, imlement Display for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // customize so only x and y are denoted
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("compare structures:");
    println!("display: {}", minmax);
    println!("debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "the big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );
    let point = Point2D { x: 3.3, y: 7.2 };

    println!("compare points:");
    println!("display: {}", point);
    println!("debug: {:?}", point);
}
