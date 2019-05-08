/*
 * pizip
 *
 * (C) 2019 Nathan Mentley <nathanmentley@gmail.com>
 *
 * This code is licnesed under the AGPL.
 */

extern crate clap;
use clap::{ArgMatches, App};

// abstract command interface
pub trait ICommand {
    fn get_name(&self) -> &str;
    fn get_subcommand<'a, 'b>(&self) -> App<'a, 'b>;
    fn execute(&self, args: &ArgMatches);
}

// command delegate interface to define a command
pub trait ICommandDelegate {
    fn get_name(&self) -> &str;
    fn get_subcommand<'a, 'b>(&self) -> App<'a, 'b>;
    fn logic(&self, args: &ArgMatches);
}

// Base command struct
pub struct Command<'a> {
    delegate: &'a dyn ICommandDelegate
}

// Implement icommand functions for command struct
impl<'a> ICommand for Command<'a> {
    fn get_name(&self) -> &str {
        return self.delegate.get_name();
    }

    fn get_subcommand<'b, 'c>(&self) -> App<'b, 'c> {
        return self.delegate.get_subcommand();
    }

    fn execute(&self, args: &ArgMatches) {
        self.delegate.logic(args);
    }
}

// setup command struct for passed delegate
pub fn new_command<'a>(delegate: &'a dyn ICommandDelegate) -> Command {
    return Command{delegate}
}