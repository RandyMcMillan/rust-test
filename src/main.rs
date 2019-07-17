use std::io::Write;
use std::collections::HashMap;

include!("guess.rs");
include!("variables.rs");
include!("expressions.rs");
include!("loops.rs");
include!("fibonacci.rs");
include!("ownership.rs");
include!("slice.rs");
include!("structs_rectangle.rs");
include!("option.rs");
include!("json.rs");
include!("collections.rs");

fn main() {
    println!("Hello world!");
    println!("Select a program");
    println!("1. Guess");
    println!("2. Variables");
    println!("3. Expressions");
    println!("4. Loops");
    println!("5. Fibonacci");
    println!("6. Ownership");
    println!("7. Slice");
    println!("8. Structs/Rectangle");
    println!("9. Option");
    println!("10. JSON");
    println!("11. Collections");

    println!("Please input your choice");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid option"),
    };

    match choice {
        1 => guess(),
        2 => variables(),
        3 => expressions(),
        4 => loops(),
        5 => fibonacci(),
        6 => ownership(),
        7 => slice(),
        8 => structs_rectangle(),
        9 => option(),
        10 => json(),
        11 => collections(),
        _ => println!("Invalid option"),
    }
}
