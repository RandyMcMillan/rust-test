use std::collections::HashMap;
use std::io::Write;

use clap::{App, SubCommand, AppSettings};
use std::iter::Map;

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
include!("traits.rs");
include!("lifetimes.rs");
include!("testing.rs");

enum Commands {
    Guess,
    Variables,
    Expressions,
    Loops,
    Fibonacci,
    Ownership,
    Slice,
    Rectangle,
    Option,
    Json,
    Collections,
    Traits,
    Lifetimes,
    Testing,
}




fn main() {



    let matches = App::new("rust-test")
        .author("RichoDemus")
        .version("v1.0-beta")
        .setting(AppSettings::ArgRequiredElseHelp)
//        .subcommands(commands)
        .subcommand(SubCommand::with_name("guess"))
        .subcommand(SubCommand::with_name("variables"))
        .subcommand(SubCommand::with_name("expressions"))
        .subcommand(SubCommand::with_name("loops"))
        .subcommand(SubCommand::with_name("fibonacci"))
        .subcommand(SubCommand::with_name("ownership"))
        .subcommand(SubCommand::with_name("slice"))
        .subcommand(SubCommand::with_name("rectangle"))
        .subcommand(SubCommand::with_name("option"))
        .subcommand(SubCommand::with_name("json"))
        .subcommand(SubCommand::with_name("collections"))
        .subcommand(SubCommand::with_name("traits"))
        .subcommand(SubCommand::with_name("lifetimes"))
        .subcommand(SubCommand::with_name("testing"))
        .get_matches();


    match matches.subcommand() {
        ("guess", Some(_)) => guess(),
        ("variables", Some(_)) => variables(),
        ("expressions", Some(_)) => expressions(),
        ("loops", Some(_)) => loops(),
        ("fibonacci", Some(_)) => fibonacci(),
        ("ownership", Some(_)) => ownership(),
        ("slice", Some(_)) => slice(),
        ("rectangle", Some(_)) => structs_rectangle(),
        ("option", Some(_)) => option(),
        ("json", Some(_)) => json(),
        ("collections", Some(_)) => collections(),
        ("traits", Some(_)) => traits(),
        ("lifetimes", Some(_)) => lifetimes(),
        ("testing", Some(_)) => testing(),
        _ => panic!("Invalid option")
    };
}
