// a slice is a reference to part of a collection

fn first_word(s: &String) -> usize {
    // return the length of the first word in a string


    let bytes = s.as_bytes();

    // find the index of the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    // if there is no space, return the length of the string
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // The compiler will keep s around as we still have a pointer pointing to its data
}


fn main() {
    let mut s = String::from("hello world");

    let word_len = first_word(&s); // word_len will get a value of 5

    s.clear();

    println!("word len: {}", word_len);
    // word_len would have a value of 5 here, but there's no more string that we could
    // use the value 5 with, word_len is now invalid in this context and could cause bugs.

    // because this is context specific, it compiles fine but we might not want it to
    // work like this. What if we need to use word_len with the string? it would be good to
    // guarantee that we can.

    // The solution is String Slices
    // a string slice is a reference to part of a String and looks like this:
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // Redo word_len using this functionality

    let mut s = String::from("hello world");

    let word_len_slice = first_word_slice(&s);

    // Other slices
    // we can slice other things too

    let a = [0, 1, 2, 3, 4, 5];

    let array_slice = &a[1..3];

    assert_eq!(slice, &[2, 3])


}
