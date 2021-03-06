extern crate clap;
extern crate image;
extern crate xterm256_converter;

use xterm256_converter::*;
use std::fs::File;
use std::io::prelude::*;
use clap::App;

fn main() {
    let matches = App::new("xterm256-converter")
        .version("0.2.0")
        .about("Converts an image to a terminal color sequence.")
        .args_from_usage(
            "-o, --output=[FILE] 'Sets the output file. If absent, will print to console.'
             -u, --unicode 'If set, the resulting sequence will use unicode characters. Make it possible to encode 2 pixels into a single character, at the cost of comptability.'
                              <INPUT>              'Sets the filename of the image to convert'",
        )
        .get_matches();

    let converted = match matches.occurrences_of("unicode") {
        0 => convert_from_file(matches.value_of("INPUT").unwrap()).unwrap(),
        _ => convert_to_unicode_from_file(matches.value_of("INPUT").unwrap()).unwrap(),
    };

    match matches.value_of("output") {
        Some(value) => File::create(value).unwrap().write_all(&converted).unwrap(),
        None => print!("{}", String::from_utf8(converted).unwrap()),
    }
}
