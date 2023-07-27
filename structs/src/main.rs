struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_truncated(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // Create a struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@exampel.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);

    // Edit a struct
    user1.email = String::from("someone@example.com");
    println!("{}", user1.email);

    // Create a struct with a function
    let user2 = build_user(
        String::from("someoneelse@example.com"),
        String::from("user2"),
    );

    // Create a struct with a function that uses shorthand inside
    let user3 = build_user_truncated(String::from("another@example.com"), String::from("user3"));

    // Create a struct based off of another struct
    let template_user = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotherexample.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user_to_be_copied_from = User {
        active: true,
        username: String::from("user4"),
        email: String::from("user4@example.com"),
        sign_in_count: 1,
    };

    let better_template_user = User {
        email: String::from("more@example.com"),
        ..user_to_be_copied_from
    };
    // NOTE that user_to_be_copied_from no longer owns its values username as String does not have a copy
    // it still owns sign_in_count as int does have a copy.

    // println!("{}", user_to_be_copied_from.username); THIS FAILS

    // Tuple structs
    // tuple structs don't have names associated with their fields, only types.
    // Use these when using names for each field is redundant or verbose, or
    // when you want to name a tuple.

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Note that black and origin are different types because they are instances
    // of different tuple structs.

    // Unit-like structs without fields
    // these behave similarly to unit tuples ()
    // They can be useful when we need to implement a trait on some type
    // but don't have any data that we want to store.

    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // No need for () or {} here
    // Functionality will use this later on.
}

// A note on lifetimes

// The following won't work as the lifetime of the references passed into it. How will
// the compiler know when to drop the data?

// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }
