// if let lets us combine the if and let functionality into a less
// verbose way to handle values that match one pattern while ignoring the rest.

fn main() {
    // Here if the value is Some, we print it out. If the value is None, we do nothing.
    let config_max = Some(3u8); // The type of config_max is Option<u8>
    match config_max {
        Some(max) => println!("the maximum is configured to be {}", max),
        _ => (),
    }

    // Instead, we could write...
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the maximum is configured to be {}", max);
    } else {
        println!("Error: maximum is None");
    }

    // Think of this as saying: If `let some(max) = config_max` succeeds,
    // (ie if config_max is a Some rather than a None), then run the expression
    // with max being the value in Some(max) that equals config_max, ie the 3u8.
    // if let takes a pattern and an expression separated by an = sign.
    // It works the same as a match.
    // Here the pattern is Some(max) and the max binds to the value inside of Some.
    // We then use the max in the body of the if let block.
    // The code is not run if the pattern does not match.
}
