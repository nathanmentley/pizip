/*
 * pizip
 *
 * (C) 2019 Nathan Mentley <nathanmentley@gmail.com>
 *
 * This code is licnesed under the AGPL.
 */

use std::io::{copy, stdout, stdin};
use std::str;

extern crate clap;
use clap::{ArgMatches, SubCommand, App, Arg};

extern crate libflate;
use libflate::gzip::{Decoder};

use super::command::{ICommandDelegate};

pub struct DecompressCommand {}
impl ICommandDelegate for DecompressCommand {
    fn get_name(&self) -> &str {
        return "decompress";
    }

    fn get_subcommand<'a, 'b>(&self) -> App<'a, 'b> {
        return SubCommand::with_name(self.get_name())
            .about("decompress stdin")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")
            )
        ;
    }

    fn logic(&self, _args: &ArgMatches) {
        let mut input = stdin();
        let mut decoder = Decoder::new(&mut input).unwrap();
        copy(&mut decoder, &mut stdout()).unwrap();
    }
}