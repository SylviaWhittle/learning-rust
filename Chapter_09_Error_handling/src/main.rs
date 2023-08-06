fn main() {
    // Unrecoverable errors with panic!
    // panic!("crash and burn");
    // This exits the program and prints a message to tell us what happened

    // Here we intentionally cause a panic due to an indexing error
    // let v = vec![1, 2, 3];
    // v[99];

    // The terminal prints out a backtrace to allow us to find the
    // source of the error

    // Recoverable errors with Result
    // The Result enum has two variants: Ok and Err

    //enum Result<T, E> {
    //    Ok(T),
    //    Err(E),
    //}

    // Many functions will return a Result rather than a value.
    // When they return a Result, we need to handle the Result

    use std::fs;
    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("problem opening the file {:?}", error),
    };

    let greeting_file_result = File::open("hello.txt");

    // Matching on different errors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {:?}", e),
            },
            other_error => {
                panic!("problem opening the file {:?}", other_error);
            }
        },
    };

    // We can have a look at closures to reduce the amount of code that this takes up
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file {:?}", error);
            })
        } else {
            panic!("problem opening the file {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    // Unwrap is a shortcut for calling panic on an error and returning the
    // result on a success
    let greeting_file = File::open("hello.txt").unwrap();

    // Expect allous us to choose the panic! error message.
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propagating errors
    // Often we don't want to panic on an error, but pass it to the calling code.
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // The ? operator allows us to take a shortcut for propagating errors
    fn read_username_from_file_shortcut() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // This can be shortened even more:
    fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // The shortest way
    // std::fs actually provides a script to do this for us, fs::read_to_string which
    // does all of the above for us:
    fn read_username_from_file_sortest() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // We can use the ? operator on Option<T> as well
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    // Custom types for validation
    // Following on from the number guessing game, we can make a type that will
    // validate input without having to throw checks all over the place
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        // A constructor
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("guess value must be between 1 and 100, got {}", value);
            }

            Guess { value }
        }

        // A getter
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
