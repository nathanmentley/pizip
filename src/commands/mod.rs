/*
 * pizip
 *
 * (C) 2019 Nathan Mentley <nathanmentley@gmail.com>
 *
 * This code is licnesed under the AGPL.
 */

mod command;

mod compress;
mod decompress;

pub fn get_commands() -> [Box<command::ICommand>; 2] {
    return [
        Box::new(command::new_command(&compress::CompressCommand{})),
        Box::new(command::new_command(&decompress::DecompressCommand{}))
    ];
}