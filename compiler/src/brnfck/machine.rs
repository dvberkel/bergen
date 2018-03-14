const SIZE: usize = 30_000;

pub struct Machine {
	pointer: usize,
	cells: [u8; SIZE],
}

impl Machine {
	pub fn new() -> Machine {
		Machine { pointer: 0, cells : [0;SIZE] }
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn execute_instruction() {
		let mut machine = Machine::new();
	}
} 