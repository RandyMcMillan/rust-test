use collections::collections;
use expressions::expressions;
use fibonacci::fibonacci;
use guess::guess;
use json::json;
use lifetimes::lifetimes;
use loops::loops;
use option::option;
use ownership::ownership;
use slice::slice;
use structs_rectangle::structs_rectangle;
use subdir::another_file::hello_from_another_dir;
use testing::testing;
use traits::traits;
use variables::variables;
use smart_pointers::pointers::pointers;
use smart_pointers::refcell::refcell;
use smart_pointers::tree::tree;
use concurrency::threads::threads;

mod collections;
mod expressions;
mod fibonacci;
mod guess;
mod json;
mod lifetimes;
mod loops;
mod option;
mod ownership;
mod slice;
mod structs_rectangle;
mod subdir;
mod testing;
mod traits;
mod variables;
mod smart_pointers;
mod concurrency;

///
/// Elaborate function to sum two integers
/// # Example
/// ```
/// let sum = rust_test_lib::add(1,3);
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
        _ => panic!("Invalid option"),
    };
}
