extern crate bergen;
extern crate clap;

use std::io;
use bergen::brnfck::to_brnfck;
use bergen::parser::parse;
use clap::{Arg, App};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let matches = App::new("bergenc")
        .version("0.1.0")
        .author("Daan van Berkel")
        .about("compiler for bergen language to brainf*ck")
        .arg(Arg::with_name("source")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("source for the bergen program to compile")
            .takes_value(true))
        .get_matches();

    let filename = matches.value_of("source").unwrap();
    let file = File::open(filename).expect("file to exist");
    let mut reader = BufReader::new(file);
    let mut source: Vec<u8> = Vec::new(); 

    reader.read_to_end(&mut source).expect("to be able to read file");

    if let Ok(instructions) = parse(&source) {
        to_brnfck(&instructions, io::stdout()).expect("to write to stdout");
    }
}
