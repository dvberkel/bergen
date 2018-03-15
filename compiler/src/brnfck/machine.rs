use std::fmt::{self, Debug, Formatter};

const SIZE: usize = 30_000;

struct BuildMachine<'a> {
	instructions: &'a [Command],
	cell_pointer: usize,
	cells: [u8; SIZE],	
}

impl<'a> BuildMachine<'a> {
	fn with(instructions: &'a[Command]) -> Self {
		BuildMachine { instructions, cell_pointer: 0, cells: [0; SIZE] }
	}

	fn pointer_at(self, cell_pointer: usize) -> Self {
		BuildMachine { instructions: self.instructions, cell_pointer, cells: self.cells }
	}

	fn cell(mut self, index: usize, value: u8) -> Self {
		self.cells[index] = value;
		BuildMachine { instructions: self.instructions, cell_pointer: self.cell_pointer, cells: self.cells }
	}

	fn build(self) -> Machine<'a> {
		Machine { instruction_pointer: 0, instructions: self.instructions, cell_pointer: self.cell_pointer, cells: self.cells }
	}
}

pub struct Machine<'a> {
	instruction_pointer: usize,
	instructions: &'a[Command],
	cell_pointer: usize,
	cells: [u8; SIZE],
}

impl<'a> Machine<'a> {
	pub fn new(instructions: &'a[Command]) -> Machine<'a> {
		Machine { instruction_pointer: 0, instructions: instructions, cell_pointer: 0, cells : [0;SIZE] }
	}

	pub fn execute(mut self) -> Result<Machine<'a>, MachineError> {
		let command = self.instructions[self.instruction_pointer];
		match command {
			Command::IncrementPointer => {
				self.cell_pointer = if self.cell_pointer  < SIZE - 1 { self.cell_pointer + 1 } else { 0 }; 
			}
			Command::DecrementPointer => { 
				self.cell_pointer = if self.cell_pointer != 0 { self.cell_pointer - 1 } else { SIZE - 1 }; 
			}
			Command::Increment => { 
				let current_value = self.cells[self.cell_pointer];
				let value = if current_value != u8::max_value() { current_value + 1 } else { u8::min_value() };
				self.cells[self.cell_pointer] = value;
			}
			Command::Decrement => { 
				let current_value = self.cells[self.cell_pointer];
				let value = if current_value != u8::min_value() { current_value - 1 } else { u8::max_value() };
				self.cells[self.cell_pointer] = value;
			}
		}
		Ok(self)
	}
}

impl<'a> PartialEq for Machine<'a> {
	fn eq(&self, rhs: &Self) -> bool {
		if self.instruction_pointer != rhs.instruction_pointer { return false; }
		if self.instructions != rhs.instructions { return false; }
		if self.cell_pointer != rhs.cell_pointer { return false; }
		if !same_cells(&self.cells, &rhs.cells) { return false; }
		true
	}
}

fn same_cells(lhs: &[u8;SIZE], rhs: &[u8;SIZE]) -> bool {
	let mut index = 0;
	while index < SIZE && lhs[index] == rhs[index] {
		index += 1;
	}
	index == SIZE
}


impl<'a> Eq for Machine<'a> {}

impl<'a> Debug for Machine<'a> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		write!(f, "<{}:[", self.instruction_pointer);
		for instruction in self.instructions {
			write!(f, "{:?}", instruction)?
		}
		write!(f, "]|{};{{", self.cell_pointer)?;
		for index in 0..SIZE {
			if self.cells[index] != 0 {
				write!(f, "({},{})", index, self.cells[index])?;
			}
		}
		write!(f, "}}>")
	}
}

pub enum MachineError {
	Unknown
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
		let instructions = [Command::IncrementPointer, Command::DecrementPointer, Command::Increment, Command::Decrement];
		for (instruction, expected_machine) in vec![
			(Command::IncrementPointer, BuildMachine::with(&instructions[0..1]).pointer_at(1).build()),
			(Command::DecrementPointer, BuildMachine::with(&instructions[1..2]).pointer_at(SIZE - 1).build()),
			(Command::Increment, BuildMachine::with(&instructions[2..3]).cell(0, 1).build()),
			(Command::Decrement, BuildMachine::with(&instructions[3..4]).cell(0, u8::max_value()).build()),
		] {
			let instructions = [instruction];
			let mut machine = Machine::new(&instructions);

			if let Ok(result_machine) = machine.execute() {
				assert_eq!(result_machine, expected_machine);
			} else {
				assert!(false);
			}
		}
	}

	#[test]
	fn increment_pointer_wraps_around() {
		let instructions = [Command::IncrementPointer];			
		let mut machine = BuildMachine::with(&instructions).pointer_at(SIZE - 1).build();

		if let Ok(result_machine) = machine.execute() {
			assert_eq!(result_machine, BuildMachine::with(&instructions).pointer_at(0).build());
		} else {
			assert!(false);
		}
	}

	#[test]
	fn increment_wraps_around() {
		let instructions = [Command::Increment];			
		let mut machine = BuildMachine::with(&instructions).cell(0, u8::max_value()).build();

		if let Ok(result_machine) = machine.execute() {
			assert_eq!(result_machine, BuildMachine::with(&instructions).cell(0,0).build());
		} else {
			assert!(false);
		}
	}
} 