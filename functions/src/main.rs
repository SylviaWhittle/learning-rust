fn main() {
    println!("main function");

    print_labelled_measurement(5, 'a');

    // Use a {} expresison to set the value of y
    let y = {
        let x = 3;
        // return x + 1 because it doesn't have a ; at the end
        // the lack of a ; means it is an expression
        // in contrast `let x = 3` is a **statement**.
        x + 1
    };
    println!("y: {y}");

    // use the return value of a function to initialise a variable
    let x = five();
    println!("five: {x}");

    println!("fixe plus one: {}", plus_one(x));
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("passed in value: {value}{unit_label}");
}

fn five() -> i32 {
    // the body here is an expression whose value we want to return
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
