use std::io;
use std::io::Write;

pub(crate) fn fibonacci_u128() {
    println!("How many numbers in the Fibonacci sequence?");

    let mut count = String::new();

    io::stdin()
        .read_line(&mut count)
        .expect("Failed to read line");

    let count: u128 = count.trim().parse().expect("Please type a number!");

    fibonacci_n(count)
}

fn fibonacci_n(n: u128) {
    for number in 1..n {
        print!("{} ", fib2(number));
        io::stdout().flush().unwrap();
    }
    println!()
}

#[allow(dead_code)]
fn fib(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

fn fib2(n: u128) -> u128 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib2(n - 1) + fib2(n - 2),
    }
}
