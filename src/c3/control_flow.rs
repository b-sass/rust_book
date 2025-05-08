pub fn compare(a: i32, b: i32) {
    if a > b {
        println!("a is bigger than b");
    } else if a < b {
        println!("a is smaller than b");
    } else {
        println!("a is equal to b");
    }

    let bigger_number = if a > b { a } else { b };
    println!("Bigger number: {bigger_number}");
}

pub fn counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result -> {result}");
}

pub fn fruits() {
    let arr = ["apple", "banana", "orange", "pear", "grape"];

    for fruit in arr {
        println!("{fruit} is {} letters long.", fruit.len())
    }
}

pub fn fizz_buzz() {
    for i in (1..100) {
        if i % 3 == 0 && i % 5 == 0 {
            println!("Fizz Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}