use clap::{App, AppSettings, SubCommand};

use gnostr_lib::run;

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
    let matches = App::new("gnostr: a git+nostr workflow utility")
        //.author("		a git+nostr workflow utility")
        .version("v0.0.1")
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
        .subcommand(SubCommand::with_name("tree"))
        .subcommand(SubCommand::with_name("threads"))
        .subcommand(SubCommand::with_name("thread2"))
        .subcommand(SubCommand::with_name("either"))
        .subcommand(SubCommand::with_name("notification"))
        .subcommand(SubCommand::with_name("rest"))
        .subcommand(SubCommand::with_name("lists"))
        .subcommand(SubCommand::with_name("install"))
        .get_matches();

    match matches.subcommand_name() {
        Some(name) => run(name),
        None => panic!("no subcommand"),
    }
}
