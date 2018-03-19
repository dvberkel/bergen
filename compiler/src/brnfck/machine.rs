use std::io::{Read, Write};
use std::fmt::{self, Debug, Formatter};

const SIZE: usize = 30_000;

pub struct Machine<'a, I: 'a, O: 'a> where I: Read, O: Write {
	input: Option<Box<I>>,
	output: Option<Box<O>>,
	instruction_pointer: usize,
	instructions: &'a[Command],
	cell_pointer: usize,
	cells: [u8; SIZE],
}

impl<'a, I, O> Machine<'a, I, O> where I: Read, O: Write {
	pub fn new(instructions: &'a[Command]) -> Machine<'a, I, O> {
		Machine { input: None, output: None, instruction_pointer: 0, instructions: instructions, cell_pointer: 0, cells : [0;SIZE] }
	}
	
	pub fn io(instructions: &'a[Command], input: Box<I>, output: Box<O>) -> Machine<'a, I, O> {
		Machine { input: Some(input), output: Some(output), instruction_pointer: 0, instructions: instructions, cell_pointer: 0, cells : [0;SIZE] }
	}

	pub fn halted(&self) -> bool {
		self.instructions.len() <= self.instruction_pointer
	}

	pub fn execute(mut self) -> Result<Machine<'a, I, O>, MachineError> {
		let command = self.instructions[self.instruction_pointer];
		match command {
			Command::IncrementPointer => {
				if self.cell_pointer != SIZE - 1 {
					self.instruction_pointer += 1;
					self.cell_pointer += 1;
					Ok(self)
				} else {
					Err(MachineError::PointerIncrementOutOfBound)
				}
			}
			Command::DecrementPointer => {
				if self.cell_pointer != 0 {
					self.instruction_pointer += 1;
					self.cell_pointer -= 1;
					Ok(self)
				} else {
					Err(MachineError::PointerDecrementOutOfBound)
				}
			}
			Command::Increment => {
				let current_value = self.cells[self.cell_pointer];
				if current_value != u8::max_value() {
					self.instruction_pointer += 1;
					self.cells[self.cell_pointer] += 1;
					Ok(self)
				} else {
					Err(MachineError::CellOverflow)
				}
			}
			Command::Decrement => {
				let current_value = self.cells[self.cell_pointer];
				if current_value != u8::min_value() {
					self.instruction_pointer += 1;
					self.cells[self.cell_pointer] -= 1;
					Ok(self)
				} else {
					Err(MachineError::CellUnderflow)
				}
			}
			Command::JumpAhead => {
				let current_value = self.cells[self.cell_pointer];
				if current_value == 0 {
					if let Some(index) = self.jump_back_index(self.instruction_pointer) {
						self.instruction_pointer = index + 1;
						Ok(self)
					} else {
						Err(MachineError::UnmatchedJumpAhead)
					}
				} else {
					self.instruction_pointer += 1;
					Ok(self)
				}
			}
			Command::JumpBack => {
				let current_value = self.cells[self.cell_pointer];
				if current_value != 0 {
					if let Some(index) = self.jump_ahead_index(self.instruction_pointer) {
						self.instruction_pointer = index + 1;
						Ok(self)
					} else {
						Err(MachineError::UnmatchedJumpBack)
					}
				} else {
					self.instruction_pointer += 1;
					Ok(self)
				}
			}
			Command::Read => {
				if self.input.is_none() { return Err(MachineError::NoInput); }
				{
					let mut input = self.input.as_mut().unwrap();
					let mut buffer: [u8;1] = [0;1];
					if let Ok(size) = (*input).read(&mut buffer) {
						if size == 1 {
							self.instruction_pointer += 1;
							self.cells[self.cell_pointer] = buffer[0];
						} else {
							return Err(MachineError::NoByteRead)
						}
					} else {
						return Err(MachineError::InputError)
					}
				}
				Ok(self)
			}
			Command::Write => {
				if self.output.is_none() { return Err(MachineError::NoOutput); }
				{
					let buffer: [u8;1] = [self.cells[self.cell_pointer];1];
					let mut output = self.output.as_mut().unwrap();
					if let Ok(size) = (*output).write(&buffer) {
						if size == 1 {
							self.instruction_pointer += 1;
						} else {
							return Err(MachineError::NoByteWritten);
						}
					} else {
						return Err(MachineError::OutputError);
					}
				}
				Ok(self)
			}
		}
	}

	pub fn run(mut self) -> Result<(), MachineError> {
		while !self.halted() {
			match self.execute() {
				Ok(next_machine) => { self = next_machine; },

				Err(error) => { return Err(error); },
			}
		}
		Ok(())
	}

	fn jump_back_index(&self, start_index: usize) -> Option<usize> {
		let mut openings = 1;
		let mut index = start_index + 1;
		while index < self.instructions.len() && openings != 0 {
			match self.instructions[index] {
				Command::JumpAhead => openings += 1,
				Command::JumpBack  => openings -= 1,
				_ => {/* do nothing */},
			}
			index += 1
		}
		if index <= self.instructions.len() && openings == 0 {
			Some(index - 1)
		} else {
			None
		}
	}

	fn jump_ahead_index(&self, start_index: usize) -> Option<usize> {
		let mut closings = 1;
		let mut index: isize = start_index as isize - 1;
		while index >= 0 && closings != 0 {
			match self.instructions[index as usize] {
				Command::JumpAhead => closings -= 1,
				Command::JumpBack  => closings += 1,
				_ => {/* do nothing */},
			}
			index -= 1
		}
		if index >= 0 && closings == 0 {
			Some(index as usize + 1)
		} else {
			None
		}
	}
}

impl<'a, I, O> PartialEq for Machine<'a, I, O> where I: Read, O: Write {
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


impl<'a, I, O> Eq for Machine<'a, I, O> where I: Read, O: Write {}

impl<'a, I, O> Debug for Machine<'a, I, O>  where I: Read, O: Write{
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		write!(f, "<{}:[", self.instruction_pointer)?;
		for instruction in self.instructions {
			write!(f, " {:?}", instruction)?;
		}
		write!(f, " ]|{};{{", self.cell_pointer)?;
		for index in 0..SIZE {
			if self.cells[index] != 0 {
				write!(f, "({},{})", index, self.cells[index])?;
			}
		}
		write!(f, "}}>")
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum MachineError {
	PointerIncrementOutOfBound,
	PointerDecrementOutOfBound,
	CellOverflow,
	CellUnderflow,
	UnmatchedJumpAhead,
	UnmatchedJumpBack,
	NoInput,
	InputError,
	NoByteRead,
	NoOutput,
	OutputError,
	NoByteWritten,	
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
	IncrementPointer,
	DecrementPointer,
	Increment,
	Decrement,
	JumpAhead,
	JumpBack,
	Read,
	Write,
}

#[cfg(test)]
mod tests {
	use super::*;

	struct BuildMachine<'a> {
		instruction_pointer: usize,
		instructions: &'a [Command],
		cell_pointer: usize,
		cells: [u8; SIZE],	
	}

	impl<'a> BuildMachine<'a> {
		fn with(instructions: &'a[Command]) -> Self {
			BuildMachine { instruction_pointer:0, instructions, cell_pointer: 0, cells: [0; SIZE] }
		}

		fn instruction_pointer_at(self, instruction_pointer: usize) -> Self {
			BuildMachine { instruction_pointer, instructions: self.instructions, cell_pointer: self.cell_pointer, cells: self.cells }
		}

		fn cell_pointer_at(self, cell_pointer: usize) -> Self {
			BuildMachine { instruction_pointer: self.instruction_pointer, instructions: self.instructions, cell_pointer, cells: self.cells }
		}

		fn cell(mut self, index: usize, value: u8) -> Self {
			self.cells[index] = value;
			BuildMachine { instruction_pointer: self.instruction_pointer, instructions: self.instructions, cell_pointer: self.cell_pointer, cells: self.cells }
		}

		fn build(self) -> Machine<'a, &'a [u8], Vec<u8>> {
			Machine { input: None, output: None, instruction_pointer: self.instruction_pointer, instructions: self.instructions, cell_pointer: self.cell_pointer, cells: self.cells }
		}
	}

	#[test]
	fn execute_increment_pointer_will_result_in_a_machine() {
		let instructions = [Command::IncrementPointer];
		let expected_machine = BuildMachine::with(&instructions).instruction_pointer_at(1).cell_pointer_at(1).build();
		let machine: Machine<&[u8], Vec<u8>> = Machine::new(&instructions);

		if let Ok(result_machine) = machine.execute() {
			assert_eq!(result_machine, expected_machine);
		} else {
			assert!(false);
		}
	}

	#[test]
	fn execute_increment_will_result_in_a_machine() {
		let instructions = [Command::Increment];
		let expected_machine = BuildMachine::with(&instructions).instruction_pointer_at(1).cell(0, 1).build();
		let machine: Machine<&[u8], Vec<u8>> = Machine::new(&instructions);

		if let Ok(result_machine) = machine.execute() {
			assert_eq!(result_machine, expected_machine);
		} else {
			assert!(false);
		}
	}

	#[test]
	fn execute_instruction_will_result_in_an_error() {
		for (instruction, expected_error) in vec![
			(Command::DecrementPointer, MachineError::PointerDecrementOutOfBound),
			(Command::Decrement, MachineError::CellUnderflow),
			(Command::Read, MachineError::NoInput),
			(Command::Write, MachineError::NoOutput),
		] {
			let instructions = [instruction];
			let machine: Machine<&[u8], Vec<u8>> = Machine::new(&instructions);

			if let Err(result_error) = machine.execute() {
				assert_eq!(result_error, expected_error);
			} else {
				assert!(false);
			}
		}
	}

	#[test]
	fn increment_pointer_should_error_when_on_boundary() {
		let instructions = [Command::IncrementPointer];			
		let machine = BuildMachine::with(&instructions).cell_pointer_at(SIZE - 1).build();

		if let Err(result_error) = machine.execute() {
			assert_eq!(result_error, MachineError::PointerIncrementOutOfBound);
		} else {
			assert!(false);
		}
	}

	#[test]
	fn increment_should_error_when_on_around() {
		let instructions = [Command::Increment];			
		let machine = BuildMachine::with(&instructions).cell(0, u8::max_value()).build();

		if let Err(result_error) = machine.execute() {
			assert_eq!(result_error, MachineError::CellOverflow);
		} else {
			assert!(false);
		}
	}
	
	#[test]
	fn jump_ahead_should_error_when_missing_jump_back() {
		let instructions = [Command::JumpAhead];			
		let machine: Machine<&[u8], Vec<u8>> = Machine::new(&instructions);

		if let Err(result_error) = machine.execute() {
			assert_eq!(result_error, MachineError::UnmatchedJumpAhead);
		} else {
			assert!(false);
		}
	}
	
	#[test]
	fn jump_back_should_error_when_missing_jump_ahead() {
		let instructions = [Command::JumpBack];			
		let machine = BuildMachine::with(&instructions).cell(0, 1).build();

		if let Err(result_error) = machine.execute() {
			assert_eq!(result_error, MachineError::UnmatchedJumpBack);
		} else {
			assert!(false);
		}
	}

	#[test]
	fn jumping_should_work_correctly() {
		let instructions = [Command::Increment, Command::Increment, Command::JumpAhead, Command::Decrement, Command::JumpBack];
		let machine: Machine<&[u8], Vec<u8>> = Machine::new(&instructions);

		if let Ok(result_machine) = machine.execute()
			.and_then(|machine| {
				assert_eq!(machine, BuildMachine::with(&instructions).instruction_pointer_at(1).cell(0, 1).build());
				machine.execute()
			}).and_then(|machine| {
				assert_eq!(machine, BuildMachine::with(&instructions).instruction_pointer_at(2).cell(0, 2).build());
				machine.execute()
			}).and_then(|machine| {
				assert_eq!(machine, BuildMachine::with(&instructions).instruction_pointer_at(3).cell(0, 2).build());
				machine.execute()
			}).and_then(|machine| {
				assert_eq!(machine, BuildMachine::with(&instructions).instruction_pointer_at(4).cell(0, 1).build());
				machine.execute()
			}).and_then(|machine| {
				assert_eq!(machine, BuildMachine::with(&instructions).instruction_pointer_at(3).cell(0, 1).build());
				machine.execute()
			}).and_then(|machine| {
				assert_eq!(machine, BuildMachine::with(&instructions).instruction_pointer_at(4).build());
				machine.execute()
			}) {
			assert_eq!(result_machine, BuildMachine::with(&instructions).instruction_pointer_at(5).build());
		} else {
			assert!(false);
		}
	}

	#[test]
	fn machine_should_not_have_halted_when_there_are_instructions_left() {
		let instructions = [Command::Increment, Command::Increment, Command::JumpAhead, Command::Decrement, Command::JumpBack];
		let machine = BuildMachine::with(&instructions).build();

		assert!(!machine.halted());
	}

	#[test]
	fn machine_should_have_halted_when_there_are_no_instructions_left() {
		let instructions = [Command::Increment, Command::Increment, Command::JumpAhead, Command::Decrement, Command::JumpBack];
		let machine = BuildMachine::with(&instructions).instruction_pointer_at(5).build();

		assert!(machine.halted());
	}
} 
