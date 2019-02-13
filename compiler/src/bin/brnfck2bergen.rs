extern crate bergen;
extern crate clap;

use bergen::brnfck::to_bergen;
use bergen::brnfck::parser::parse;
use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

fn main() {
    let matches = App::new("brnfck2bergen")
        .version("0.1.0")
        .author("Daan van Berkel")
        .about("convert brainf*ck programs to bergen programs")
        .arg(
            Arg::with_name("source")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("source for the brnfck program to transpile")
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("source").unwrap();
    let file = File::open(filename).expect("file to exist");
    let mut reader = BufReader::new(file);
    let mut source: Vec<u8> = Vec::new();

    reader
        .read_to_end(&mut source)
        .expect("to be able to read file");

    if let Ok(instructions) = parse(&source) {
        to_bergen(&instructions, io::stdout()).expect("to be able to write bergen");
    }
}

