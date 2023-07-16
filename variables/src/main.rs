fn main() {
    let mut x = 5;
    println!("x: {x}");
    x = 6;
    println!("x: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const: {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("the value of y in the inner scope is {y}");
    }

    println!("the value of y in the outer scope is {y}");

    // numeric operations
    let sum = 5 + 10;
    println!("sum: {sum}");

    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    let product = 3 * 30;
    println!("product: {product}");

    let quotient = 56.7 / 32.2;
    let truncated = 5 / 3;
    println!("quotient: {quotient} truncated: {truncated}");

    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    let tup: (i32, f64, u8) = (500, 5.0, 5);
    // println!("tup: {tup}"); <- DOESN'T WORK AS NOT IMPLEMENTED
    // destructure the tuple
    let (x, y, z) = tup;
    println!("tup: x:{x} y:{y} z:{z}");
    println!("first element of tup: {}", tup.0);

}
