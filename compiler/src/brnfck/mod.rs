mod machine;

pub use self::machine::{Command, Machine, MachineError};

pub fn run(instructions: &[machine::Command]) -> Result<(), machine::MachineError> {
	let mut machine: machine::Machine<&[u8], Vec<u8>> = machine::Machine::new(instructions);

	machine.run()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_simple_program() {
		let instructions = [Command::Increment, Command::Increment, Command::JumpAhead, Command::Decrement, Command::JumpBack];
		
		assert_eq!(run(&instructions), Ok(()));
	}
}