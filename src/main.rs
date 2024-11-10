use crate::app::App;
use crate::cli::Commands;
use clap::Parser;
use cli::Cli;
use color_eyre::Result;

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod logging;
mod tui;

#[tokio::main]
async fn tui() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}

use std::process;
fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.gnostr_tui {
        0 => {
            //println!("gnostr_tui 0");
            //process::exit(0);

        }
        1 => {
            println!("gnostr_tui 1");
            gnostr_lib::run("gnostr-tui", "");
            process::exit(0);
        }
        2 => {
            println!("gnostr_tui 2");
            gnostr_lib::run("gnostr-tui", "");
            process::exit(0);
        }
        _ => {
            println!("gnostr_tui -");
            //process::exit(0);
        }
    }

    match &cli.command {
        Some(Commands::Add { name }) => {
            println!("{:}", name.as_ref().unwrap_or(&String::from("")));
            print!("{:}", name.clone().unwrap_or(String::from("")));
        }
        Some(Commands::Install { name }) => {
            if let Some(..) = name {
                gnostr_lib::run("install", name.as_ref().expect(""));
            } else {
                gnostr_lib::run("install", "help");
            }
        }
        Some(Commands::CargoInstall { name }) => {
            if let Some(..) = name {
                gnostr_lib::run("cargo-install", name.as_ref().expect(""));
            } else {
                gnostr_lib::run("cargo-install", "help");
            }
        }
        Some(Commands::GnostrTui { tui }) => {
            if let Some(..) = tui {
                gnostr_lib::run("gnostr-tui", "");
            } else {
                gnostr_lib::run("gnostr-tui", "help");
            }
        }
        None => {
            println!("Default:None");
            let _ = tui();
        }
    }
}
