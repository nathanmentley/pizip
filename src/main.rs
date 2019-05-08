/*
 * pizip
 *
 * (C) 2019 Nathan Mentley <nathanmentley@gmail.com>
 *
 * This code is licnesed under the AGPL.
 */

extern crate clap;
use clap::{App};

mod commands;
mod compression;
use commands::{get_commands};

fn main() {
    //setup app
    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));

    //register commands
    let commands = get_commands();
    for command in &commands {
        app = app.subcommand(command.get_subcommand());
    }

    //read execute context and act on the correct command(s).
    let matches = app.get_matches();
    for command in &commands {
        if let Some(matches) = matches.subcommand_matches(command.get_name()) {
            command.execute(matches);
        }
    }
}