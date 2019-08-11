use clap::{App, AppSettings, SubCommand};

use rust_test_lib::run;

// todo try those iterable enums or something
//enum Commands {
//    Guess,
//    Variables,
//    Expressions,
//    Loops,
//    Fibonacci,
//    Ownership,
//    Slice,
//    Rectangle,
//    Option,
//    Json,
//    Collections,
//    Traits,
//    Lifetimes,
//    Testing,
//    Subdir,
//}

fn main() {
    let matches = App::new("rust-test")
        .author("RichoDemus")
        .version("v1.0-beta")
        .setting(AppSettings::ArgRequiredElseHelp)
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
        .subcommand(SubCommand::with_name("subdir"))
        .subcommand(SubCommand::with_name("pointers"))
        .subcommand(SubCommand::with_name("refcell"))
        .get_matches();

    match matches.subcommand_name() {
        Some(name) => run(name),
        None => panic!("no subcommand"),
    }
}
