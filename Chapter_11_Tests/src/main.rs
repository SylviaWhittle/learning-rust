fn main() {
    println!("Hello, world!");
}

// Tests behave very similarly to pytest tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_assert() {
        assert!(true, "custom message!");
    }

    #[test]
    #[should_panic(expected = "panic message!")]
    fn test_should_panic() {
        panic!("panic message!");
    }
}
