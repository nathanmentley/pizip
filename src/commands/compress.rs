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
use clap::{ArgMatches, SubCommand, App};

extern crate libflate;
use libflate::gzip::{Encoder};

use super::command::{ICommandDelegate};
use super::super::compression::{pi::pihex};

pub struct CompressCommand {}
impl ICommandDelegate for CompressCommand {
    fn get_name(&self) -> &str {
        return "compress";
    }

    fn get_subcommand<'a, 'b>(&self) -> App<'a, 'b> {
        return SubCommand::with_name(self.get_name())
            .about("compress stdin")
        ;
    }

    fn logic(&self, _args: &ArgMatches) {
        pihex(1200);
        let mut encoder = Encoder::new(Vec::new()).unwrap();
        copy(&mut stdin().lock(), &mut encoder).unwrap();
        let encoded_data = encoder.finish().into_result().unwrap();
        copy(&mut encoded_data.as_slice(), &mut stdout()).unwrap();
    }
}