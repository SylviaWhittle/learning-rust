fn main() {
    let number = 3;

    if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is greater or equal to 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("condition: {condition} number: {number}");
}
