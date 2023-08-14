// Closures are like functions, but can capture variables
// from the enclosing scope.
// let x = 4;
// let adder |y| x + y;
// let result = adder(2)
// --> 6
// Here we use closures to make code more concise:
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Use a closure to capture the self instance
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "the user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure type inference and annotation
    // Closures don't require type annotation as they are never user-facing
    // and so can be inferred by the compiler at compile time

    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1} ;
    // let add_one_v4 = |x| x + 1;

    // Note that closures can capture values in three ways:
    // - borrowing immutably,
    // - borrowing mutably,
    // - taking ownership

    // These will have restrictions depending on context to
    // ensure memory safety.

    // Iterators

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // Note that this doesn't unpack the values, but similar to
    // python generators, unpacks them when needed.
    for val in v1_iter {
        println!("iteration value: {}", val);
    }

    // All iterators implement the next() method.
    // Here's the code for the iterator trait:
    // pub trait Iterator {
    //     type Item;

    //     fn next(&mut self) -> Option<Self::Item>;

    //     // methods with default implementations elided
    // }

    // Note that type Item and Self::Item are using associated typing.
    // This just means that in order to implement the iterator trait,
    // we also need to define an Item type and this Item type is used in
    // the return type of the next method. Ie, the Item type will be the
    // type returned by the iterator.

    // next() can be called on iterators directly to get the next element
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // Note that .next() mutates the internal state of the iterator and
    // so we need to make it a mutable variable

    // The values we get from the iterator are immutable references to the values
    // in the vector.
    // If we want an iterator that takes ownership of v1 and returns owned values,
    // we can use the into_iter() function.
    // If we want to iterate over mutable references, we can call iter_mut().

    // Iterator has a few methods which destroy the iterator because they use .next()
    // inside them
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // Iterator adaptors are methods defined on the Iterator trait that produce different
    // iterators by changing parts of the original.

    // The .map() method takes a closure to call each item on and returns an iterator that
    // produces the modified items.

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);
    for val in v1_iter {
        println!("{}", val);
    }

    // See an example in lib.rs (it's there since lib.rs tests have access to everything without
    // it needing to be public.)
}
