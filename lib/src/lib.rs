use collections::collections;
use concurrency::thread2::thread2;
use concurrency::threads::threads;
use either::either_usage::either;
use expressions::expressions;
use fibonacci::fibonacci;
use guess::guess;
use json::json;
use lifetimes::lifetimes;
use loops::loops;
use making_lists::ring_buffer;
use notification::notification;
use option::option;
use ownership::ownership;
use rest::rest;
use slice::slice;
use smart_pointers::pointers::pointers;
use smart_pointers::refcell::refcell;
use smart_pointers::tree::tree;
use structs_rectangle::structs_rectangle;
use subdir::another_file::hello_from_another_dir;
use testing::testing;
use traits::traits;
use variables::variables;

use install::install;
mod install;

mod collections;
mod concurrency;
mod either;
mod expressions;
mod fibonacci;
mod guess;
mod json;
mod lifetimes;
mod loops;
mod making_lists;
mod notification;
mod option;
mod ownership;
mod rest;
mod slice;
mod smart_pointers;
mod structs_rectangle;
mod subdir;
mod testing;
mod traits;
mod variables;

///
/// Elaborate function to sum two integers
/// # Example
/// ```
/// let sum = gnostr_lib::add(1,3);
/// assert_eq!(4, sum);
/// ```

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run(name: &str) {
    match name {
        "guess" => guess(),
        "variables" => variables(),
        "expressions" => expressions(),
        "loops" => loops(),
        "fibonacci" => fibonacci(),
        "ownership" => ownership(),
        "slice" => slice(),
        "rectangle" => structs_rectangle(),
        "option" => option(),
        "json" => json(),
        "collections" => collections(),
        "traits" => traits(),
        "lifetimes" => lifetimes(),
        "testing" => testing(),
        "subdir" => hello_from_another_dir(),
        "pointers" => pointers(),
        "refcell" => refcell(),
        "tree" => tree(),
        "threads" => threads(),
        "thread2" => thread2(),
        "either" => either(),
        "notification" => notification(),
        "rest" => rest(),
        "lists" => ring_buffer::ring_buffer(),
        "install" => install(),
        _ => panic!("Invalid option"),
    };
}
