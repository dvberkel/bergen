use std::fmt::{self, Debug, Formatter};

const SIZE: usize = 30_000;

struct BuildMachine<'a> {
	instructions: &'a [Command],
	pointer: usize,
	cells: [u8; SIZE],	
}

impl<'a> BuildMachine<'a> {
	fn with(instructions: &'a [Command]) -> Self {
		BuildMachine { instructions, pointer: 0, cells: [0; SIZE] }
	}

	fn pointer_at(self, pointer: usize) -> Self {
		BuildMachine { instructions: self.instructions, pointer, cells: self.cells }
	}

	fn cell(mut self, index: usize, value: u8) -> Self {
		self.cells[index] = value;
		BuildMachine { instructions: self.instructions, pointer: self.pointer, cells: self.cells }
	}

	fn build(self) -> Machine {
		Machine { pointer: self.pointer, cells: self.cells }
	}
}

pub struct Machine {
	pointer: usize,
	cells: [u8; SIZE],
}

impl Machine {
	pub fn new() -> Machine {
		Machine { pointer: 0, cells : [0;SIZE] }
	}

	fn pointer_at(mut self, value: usize) -> Machine {
		self.pointer = value;

		self
	}

	pub fn execute(mut self, command: Command) -> Result<Machine, MachineError> {
		match command {
			Command::IncrementPointer => {
				self.pointer = if self.pointer  < SIZE - 1 { self.pointer + 1 } else { 0 }; 
			}
			Command::DecrementPointer => { 
				self.pointer = if self.pointer != 0 { self.pointer - 1 } else { SIZE - 1 }; 
			}
			Command::Increment        => { 
				let current_value = self.cells[self.pointer];
				let value = if current_value != u8::max_value() { current_value + 1 } else { u8::min_value() };
				self.cells[self.pointer] = value;
			}
			Command::Decrement        => { 
				let current_value = self.cells[self.pointer];
				let value = if current_value != u8::min_value() { current_value - 1 } else { u8::max_value() };
				self.cells[self.pointer] = value;
			}
		}
		Ok(self)
	}
}

impl PartialEq for Machine {
	fn eq(&self, rhs: &Self) -> bool {
		if self.pointer.eq(&rhs.pointer) {
			let mut index = 0;
			while index < SIZE && self.cells[index].eq(&rhs.cells[index]) {
				index += 1;
			}
			index == SIZE
		} else {
			false
		}
	}
}

impl Eq for Machine {}

impl Debug for Machine {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		write!(f, "<{};[", self.pointer)?;
		for index in 0..SIZE {
			if self.cells[index] != 0 {
				write!(f, "({},{})", index, self.cells[index])?;
			}
		}
		write!(f, "]>")
	}
}

pub enum MachineError {
	Unknown
}

pub enum Command {
	IncrementPointer,
	DecrementPointer,
	Increment,
	Decrement,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn execute_instruction() {
		for (instruction, expected_machine) in vec![
			(Command::IncrementPointer, BuildMachine::with(&[]).pointer_at(1).build()),
			(Command::DecrementPointer, BuildMachine::with(&[]).pointer_at(SIZE - 1).build()),
			(Command::Increment, BuildMachine::with(&[]).cell(0, 1).build()),
			(Command::Decrement, BuildMachine::with(&[]).cell(0, u8::max_value()).build()),
		] {
			let mut machine = Machine::new();

			if let Ok(result_machine) = machine.execute(instruction) {
				assert_eq!(result_machine, expected_machine);
			} else {
				assert!(false);
			}
		}
	}

	#[test]
	fn increment_pointer_wraps_around() {
			let mut machine = BuildMachine::with(&[]).pointer_at(SIZE - 1).build();

			if let Ok(result_machine) = machine.execute(Command::IncrementPointer) {
				assert_eq!(result_machine, BuildMachine::with(&[]).pointer_at(0).build());
			} else {
				assert!(false);
			}

	}

	#[test]
	fn increment_wraps_around() {
			let mut machine = BuildMachine::with(&[]).cell(0, u8::max_value()).build();

			if let Ok(result_machine) = machine.execute(Command::Increment) {
				assert_eq!(result_machine, BuildMachine::with(&[]).cell(0,0).build());
			} else {
				assert!(false);
			}

	}
} 