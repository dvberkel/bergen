use std::io::{self, Read, Write};
use std::ops::Add;

const NEWLINE: [u8;1] = [10];

mod machine;
pub mod parser;

pub use self::machine::{Command, MachineError};

pub fn run(instructions: &[machine::Command]) -> Result<(), machine::MachineError> {
    let machine: machine::Machine<&[u8], Vec<u8>> = machine::Machine::new(instructions);

    machine.run()
}

pub fn io_run<I: Read, O: Write>(
    instructions: &[machine::Command],
    input: I,
    output: O,
) -> Result<(), machine::MachineError> {
    let input_box = Box::new(input);
    let output_box = Box::new(output);
    let machine: machine::Machine<I, O> = machine::Machine::io(instructions, input_box, output_box);

    machine.run()
}

pub fn to_brnfck<O: Write>(
    instructions: &[machine::Command],
    mut output: O,
) -> Result<(), io::Error> {
    let mut program = String::new();
    for instruction in instructions {
        program.push(instruction.to_brnfck())
    }

    output.write_all(program.as_bytes())
}

pub fn to_bergen<O: Write>(instructions: &[machine::Command], mut output: O) -> Result<(), io::Error> {
	let (mut top, mut middle, mut bottom) = (String::new(), String::new(), String::new());
	for instruction in instructions {
		top = top + instruction.top();
		middle = middle + instruction.middle();
		bottom = bottom + instruction.bottom();
	}

	output.write_all(top.as_bytes())?;
	output.write_all(&NEWLINE)?;
	output.write_all(middle.as_bytes())?;
	output.write_all(&NEWLINE)?;
	output.write_all(bottom.as_bytes())?;
	output.write_all(&NEWLINE)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_simple_program() {
        let instructions = [
            Command::Increment,
            Command::Increment,
            Command::JumpAhead,
            Command::Decrement,
            Command::JumpBack,
        ];

        assert_eq!(run(&instructions), Ok(()));
    }

    #[test]
    fn run_io_program() {
        let instructions = [Command::Read, Command::Write];
        let input: &[u8] = "a".as_bytes();
        let mut output: Vec<u8> = vec![];

        assert_eq!(io_run(&instructions, input, &mut output), Ok(()));
        assert_eq!(output, vec!(97));
    }
}
