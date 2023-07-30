#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // Note that with match statements, unlike if statements,
    // each case does not need to evaluate to a boolean.
    // The => operator separates the case and the code to run.
    // Each case here returns the value after the =>.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // We need the Some here since there is an option to reutrn
        // None in another branch in this match.
        Some(i) => Some(i + 1),
    }
}

fn main() {
    value_in_cents(Coin::Quarter(USState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matching like this is very common in Rust, matching to
    // an enum, binding a variable based on the data inside,
    // and executing code based on the value.

    // Matches are exhaustive
    // The arms must cover all possibilities.
    // This won't compile:

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // This is because we didn't handle the None case.
    // This is good because it catches errors before runtime.

    // Catch-all patterns and the _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // we could also use:
        // _ => reroll(),
        // or do nothing:
        // _ => ()
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}
