fn main() {
    let mut counter = 0;

    let result = loop {
        println!("looping");
        counter += 1;
        println!("incrementing counter");

        if counter == 10 {
            println!("breaking loop and returning at counter {counter}");
            break counter * 2;
        }
    };

    println!("result is {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count: {count}");

    println!("------");

    let mut count = 0;

    while count < 5 {
        println!("count: {count}");

        count += 1;
    }

    println!("-------");

    // for loops iterate over a range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("bam!");
}
