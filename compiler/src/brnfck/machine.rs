use std::fmt::{self, Debug, Formatter};

const SIZE: usize = 30_000;

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

	fn build(self) -> Machine<'a> {
		Machine { instruction_pointer: self.instruction_pointer, instructions: self.instructions, cell_pointer: self.cell_pointer, cells: self.cells }
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
				self.instruction_pointer += 1;
				self.cell_pointer = if self.cell_pointer  < SIZE - 1 { self.cell_pointer + 1 } else { 0 }; 
			}
			Command::DecrementPointer => {
				self.instruction_pointer += 1;
				self.cell_pointer = if self.cell_pointer != 0 { self.cell_pointer - 1 } else { SIZE - 1 }; 
			}
			Command::Increment => {
				self.instruction_pointer += 1;
				let current_value = self.cells[self.cell_pointer];
				let value = if current_value != u8::max_value() { current_value + 1 } else { u8::min_value() };
				self.cells[self.cell_pointer] = value;
			}
			Command::Decrement => {
				self.instruction_pointer += 1;
				let current_value = self.cells[self.cell_pointer];
				let value = if current_value != u8::min_value() { current_value - 1 } else { u8::max_value() };
				self.cells[self.cell_pointer] = value;
			}
			Command::JumpAhead => {
				let current_value = self.cells[self.cell_pointer];
				self.instruction_pointer = if current_value == 0 {
					self.jump_back_index(self.instruction_pointer).unwrap() /* TODO correctly handle None */
				} else {
					self.instruction_pointer + 1
				}
			}
			Command::JumpBack => {
				let current_value = self.cells[self.cell_pointer];
				self.instruction_pointer = if current_value != 0 {
					self.jump_ahead_index(self.instruction_pointer).unwrap() /* TODO correctly handle None */
				} else {
					self.instruction_pointer + 1
				}
			}
		}
		Ok(self)
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
		let mut index = start_index - 1;
		while index >= 0 && closings != 0 {
			match self.instructions[index] {
				Command::JumpAhead => closings -= 1,
				Command::JumpBack  => closings += 1,
				_ => {/* do nothing */},
			}
			index -= 1
		}
		if index >= 0 && closings == 0 {
			Some(index + 1)
		} else {
			None
		}
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
		write!(f, "<{}:[", self.instruction_pointer)?;
		for instruction in self.instructions {
			write!(f, "{:?}", instruction)?;
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
	JumpAhead,
	JumpBack,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn execute_instruction() {
		let instructions = [Command::IncrementPointer, Command::DecrementPointer, Command::Increment, Command::Decrement];
		for (instruction, expected_machine) in vec![
			(Command::IncrementPointer, BuildMachine::with(&instructions[0..1]).instruction_pointer_at(1).cell_pointer_at(1).build()),
			(Command::DecrementPointer, BuildMachine::with(&instructions[1..2]).instruction_pointer_at(1).cell_pointer_at(SIZE - 1).build()),
			(Command::Increment, BuildMachine::with(&instructions[2..3]).instruction_pointer_at(1).cell(0, 1).build()),
			(Command::Decrement, BuildMachine::with(&instructions[3..4]).instruction_pointer_at(1).cell(0, u8::max_value()).build()),
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
		let mut machine = BuildMachine::with(&instructions).cell_pointer_at(SIZE - 1).build();

		if let Ok(result_machine) = machine.execute() {
			assert_eq!(result_machine, BuildMachine::with(&instructions).instruction_pointer_at(1).cell_pointer_at(0).build());
		} else {
			assert!(false);
		}
	}

	#[test]
	fn increment_wraps_around() {
		let instructions = [Command::Increment];			
		let mut machine = BuildMachine::with(&instructions).cell(0, u8::max_value()).build();

		if let Ok(result_machine) = machine.execute() {
			assert_eq!(result_machine, BuildMachine::with(&instructions).instruction_pointer_at(1).cell(0,0).build());
		} else {
			assert!(false);
		}
	}

	#[test]
	fn jumping_should_work_correctly() {
		let instructions = [Command::Increment, Command::Increment, Command::JumpAhead, Command::Decrement, Command::JumpBack];
		let mut machine0 = Machine::new(&instructions);

		if let Ok(machine1) = machine0.execute() {
			assert_eq!(machine1, BuildMachine::with(&instructions).instruction_pointer_at(1).cell(0, 1).build());
			if let Ok(machine2) = machine1.execute() {
				assert_eq!(machine2, BuildMachine::with(&instructions).instruction_pointer_at(2).cell(0, 2).build());
				if let Ok(machine3) = machine2.execute() {
					assert_eq!(machine3, BuildMachine::with(&instructions).instruction_pointer_at(3).cell(0, 2).build());
					if let Ok(machine4) = machine3.execute() {
						assert_eq!(machine4, BuildMachine::with(&instructions).instruction_pointer_at(4).cell(0, 1).build());
						if let Ok(machine5) = machine4.execute() {
							assert_eq!(machine5, BuildMachine::with(&instructions).instruction_pointer_at(2).cell(0, 1).build());
							if let Ok(machine6) = machine5.execute() {
								assert_eq!(machine6, BuildMachine::with(&instructions).instruction_pointer_at(3).cell(0, 1).build());
								if let Ok(machine7) = machine6.execute() {
									assert_eq!(machine7, BuildMachine::with(&instructions).instruction_pointer_at(4).build());
									if let Ok(machine8) = machine7.execute() {
										assert_eq!(machine8, BuildMachine::with(&instructions).instruction_pointer_at(5).build());
									} else {
										assert!(false)
									}
								} else {
									assert!(false)
								}
							} else {
								assert!(false)
							}
						} else {
							assert!(false)
						}
					} else {
						assert!(false)
					}
				} else {
					assert!(false)
				}
			} else {
				assert!(false)
			}
		} else {
			assert!(false)
		}
	}
} 