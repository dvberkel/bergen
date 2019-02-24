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

pub fn program_from(characters: &[u8]) -> Vec<Command> {
    let mut program = Vec::new();
    let mut index = 0;
    let mut last_character = 0;
    while index < characters.len() {
        let mut difference = characters[index] as i16 - last_character as i16;
        let command = if difference > 0 { Command::Increment } else { Command::Decrement };
        difference = difference.abs();
        if difference >= 2 {
            let factors = factors_of(difference);
            change_to_character_by(factors.len(), &factors, command, &mut program);
        } else if difference == 1 {
            program.push(command);
        } else {
            /* do nothing; difference == 0 */
        }
        program.push(Command::Write);
        last_character = characters[index];
        index += 1;
    }

    program
}

fn change_to_character_by(n: usize, factors: &[i16], command: Command, program : &mut Vec<Command>)  {
    if factors.len() == 0 {
        alter_to_left(n, command, program);
    } else {
        program.push(Command::IncrementPointer);
        let mut factor = factors[0];
        while factor > 0 {
            program.push(Command::Increment);
            factor -= 1;
        }
        program.push(Command::JumpAhead);
        change_to_character_by(n, &factors[1..], command, program);
        program.push(Command::Decrement);
        program.push(Command::JumpBack);
        program.push(Command::DecrementPointer);
    }
}

fn alter_to_left(n: usize, command: Command, program: &mut Vec<Command>) {
    if n == 0 {
        program.push(command);
    } else {
        program.push(Command::DecrementPointer);
        alter_to_left(n - 1, command, program);
        program.push(Command::IncrementPointer);
    }
}

fn factors_of(mut n: i16) -> Vec<i16> {
    let mut factors = Vec::new();
    let mut divisor = 2;
    while divisor <= n {
        while n % divisor == 0 {
            factors.push(divisor);
            n = n / divisor;
        }
        divisor += 1;
    }
    factors
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

    #[test]
    fn determine_prime_factors() {
        assert_eq!(factors_of(2), vec![2]);
        assert_eq!(factors_of(3), vec![3]);
        assert_eq!(factors_of(4), vec![2, 2]);
        assert_eq!(factors_of(5), vec![5]);
        assert_eq!(factors_of(6), vec![2, 3]);
     }
}
