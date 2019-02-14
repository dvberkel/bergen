extern crate bergen;
extern crate clap;

use bergen::brnfck::{program_from, to_bergen};
use clap::{App, Arg};
use std::io;

fn main() {
    let matches = App::new("say")
        .version("0.1.0")
        .author("Daan van Berkel")
        .about("convert brainf*ck programs to bergen programs")
        .arg(
            Arg::with_name("sentence")
                .short("s")
                .long("sentence")
                .value_name("STRING")
                .help("create a bergen program that outputs a sentence")
                .takes_value(true),
        )
        .get_matches();

    let sentence = matches.value_of("sentence").unwrap();
    let characters = sentence.as_bytes();

    let program = program_from(&characters);
    to_bergen(&program, io::stdout()).expect("to write program");
}

