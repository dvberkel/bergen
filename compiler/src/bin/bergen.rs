extern crate bergen;

use bergen::brnfck::run;
use bergen::parser::parse;

fn main() {
    let source = "".as_bytes();

    if let Ok(instructions) = parse(source) {
        if let Ok(_) = run(&instructions) {
            println!("Ran machine");
        }
    }
}
