fn main() {
    // Generic data types
    // We use generics to create definitions for items like function signatures
    // or structs which we can then use with many different data types.

    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("the largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("the largest char is {result}");

    // To define a generic function that can handle multiple types,
    // we can do this:
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        // This means the function is a generic over some type T.
        // The types of T are only those that implement the
        // std::cmp::PartialOrd trait that allows < > to be used on them.
        // The function has one parameter called list which is a slice of
        // values of type T.
        // The function returns a reference to a value of the same type T.
        let mut largest = &list[0];

        for item in list {
            // This won't compile if we dont restrict the types
            // since > is not defined for all types!
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let result = largest(&number_list);
    println!("the largest i32 is {result}");

    let result = largest(&char_list);
    println!("the largest char is {result}");

    // In struct definitions
    // We can also define structs to use a generic parameter in one or more
    // fields using the <> syntax.

    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    struct DifferentTypePoints<T, U> {
        x: T,
        y: U,
    }

    let mix = DifferentTypePoints { x: 5, y: 10.0 };

    // In enum definitions
    // The Option enum is an enum using generic typing:

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // In method definitions
    struct Point_generic<T> {
        x: T,
        y: T,
    }

    // Implementing for points of any type
    impl<T> Point_generic<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Implementing for points of type i32
    impl Point_generic<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point_generic { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // Traits : Defining shared behaviour

    pub trait Summary {
        fn summarize(&self) -> String {
            // This is the default behaviour if types don't
            // implement it specifically.
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // impl Summary for NewsArticle {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Default implementations
    // To use a default implementation like the one for Summary
    // above, we can do this:
    impl Summary for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Traits as parameters

    // Define a notify function that calls the summarize method on its item
    // parameter which is of some type that implements teh Summary trait.
    pub fn notify(item: &impl Summary) {
        // This ensures that the parameter implements Summary
        println!("breakding news, {}", item.summarize());
    }

    // The impl Trait syntax is actually syntax sugar for a longer form known as
    // trait bound. It looks like this:
    pub fn notify_verbose<T: Summary>(item: &T) {
        println!("breaking news, {}", item.summarize());
    }

    // We can have two parameters that implement Summary:
    pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {}

    // If we want them to be the same type though we need to use the verbose definition
    pub fn notify_two_same_type<T: Summary>(item1: &T, item2: &T) {}

    // Specifying multiple Trait Bounds with the + syntax
    use std::fmt::Debug;
    use std::fmt::Display;
    pub fn notify_multiple_trait_bounds(item: &(impl Summary + Display)) {}
    pub fn notify_multiple_trait_bounds_verbose<T: Summary + Display>(item: &T) {}

    // Clearer Trait Bounds with the where clause
    // It can be cumbersome to use a lot of trait bounds...
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        return 5;
    }

    fn some_function_where<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        return 5;
    }

    // Returning types that implement traits
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // Using trait bounds to conditionally implement methods
    struct Pair<T> {
        x: T,
        y: T,
    }

    // Always implement the constructor regardless of type and traits
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Only implement the cmp_display method if it is a type that
    // has the Display and PartialOrd traits.
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("the largest member is x = {}", self.x);
            } else {
                println!("the largest member is y = {}", self.y);
            }
        }
    }

    // Blanket implementations

    // We can also conditionally implement a trait for any type that implements
    // andother trait
    // For example, the standard library implements the ToString trait on any
    // type that implements the Display trait. The impl bloack in the standard
    // library looks like this:

    // impl<T: Display> ToString for T {}
    
    // Because the standard library has this blanket implementation, we can call
    // the to_string method defined by the ToString trait on any type that
    // implements the Display trait. 


    // =======================================

    // Validating references with Lifetimes

    // This won't compile due to a dangling reference, &x

    // fn main() {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    // Since &x's lifetime, `b is smaller than r's lifetime, `a, &x will be dangling
    // and refer to deallocated memory. This is not allowed and would cause errors!

    // Generic lifetimes in functions

    // This won't compile as the compiler doesn't know whether the return value
    // will be borrowed from x or y, and neither do we, so it cannot tell how
    // long it will remain valid for in memory in our program.

    // fn longest (x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);

    // Lifetime annotation syntax
    // Lifetime annotations do not change how long references live.
    // They describe the relationships of multiple references to each other without
    // affecting the lifetimes. Just as functions can accept any type when the
    // signature specifies a generic type parameter, functions can accept
    // references with any lifetime by specifying a generic lifetime parameter.

    // Lifetime annotations have a slightly unusual syntax: the names of the
    // lifetime parameters must start with an '

    // Here are some examples:
    // Reference to an i32 without a lifetime parameter:
    // &i32
    // Reference to an i32 with an explicit lifetime called 'a
    // &'a i32
    // Mutable reference to an i32 with a lifetime called 'a
    // &'a mut i32

    // Back to the "longest" function example
    // We want the signature to express the constraint:
    // "The returned reference will be valid as long as both the parameters are valid"
    // This is the relationship between lifetimes of the parameters and the return
    // value.
    // We will name the lifetime 'a and then add it to each reference:
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // The function signature now tells Rust that for some lifetime 'a, the function
    // takes two parameters, both of which are string slices that live at least as
    // long as lifetime 'a.
    // The function signature also tells Rust that the string slice returned from
    // the function will live at least as long as lifetime 'a.
    // In practice, this means that the lifetime of the values returned by the
    // function is the same as the smaller of the lifetimes of the values
    // referred to by the function arguments.

    // This is how they affect things:

    // Here string1 is valid until the end of the outer scope, and string2
    // is valid until the end of the inner scope.
    let string1  = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("the longest string is {}", result);
    }

    // This does not compile as the string2 does not live long enough
    // to be referenced outside of the inner scope.
    // Rust only knows this becase we annotated the lifetimes of the
    // variables in the longest() function to be the smallest of the
    // two arguments (for safety) since then we can be sure that
    // in either case of x or y being returned, that Rust
    // will make sure that the return value is valid when referenced.

    // Ultimately, lifetime syntax is about connecting the lifetimes of various 
    // parameters and return values of functions.
    // Once they're connected, Rust has enough information to allow memory
    // safe operations and disallow operations that would create dangling pointers.

    

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);



    
}
