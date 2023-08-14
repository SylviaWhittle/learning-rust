// Using closures that capture the environment
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // This takes ownership of the vector and returns a vector containing
    // only the shoes in the right sizes.
    // into_iter() creates an iterator that takes ownership of the vector
    // and then we call filter to adapt the iterator into a new iterator
    // that only contains the elements which the closure returns true.
    // The closure captures the shoe_size parameter from the environment
    // and compares them to the value for each shoe size, keeping only
    // the shoes of the specified size.
    // Finally the collect() method gathers the values returned into
    // a vector that is then returned.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
// Note that we have this in lib.rs because functions in lib.rs
// do not need to be public to be able to be accessed by the tests.
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
