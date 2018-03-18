use std::io::{Read, Write};

#[test]
fn read_from_buffer() {
	let mut input: &[u8] = "Hello, World!".as_bytes();
	let mut buffer: [u8;1];

	for character in vec!["H", "e", "l", "l", "o", ",", " ", "W", "o", "r", "l", "d", "!"] {
		buffer = [0];
		let result = input.read(&mut buffer);
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), 1);
		assert_eq!(buffer, character.as_bytes());
	}

	buffer = [0];
	if let Ok(size) = input.read(&mut buffer) {
		assert_eq!(size, 0);
	} else {
		assert!(false);
	}
}

#[test]
fn write_to_vec() {
	let bytes: [u8;3] = [97, 98, 99];
	let mut buffer: [u8;1];
	let mut output: Vec<u8> = vec!();
	{
		let output_ref = &mut output;

		for byte in bytes.iter() {
			buffer = [*byte];
			let result = output_ref.write(&buffer);
			assert!(result.is_ok());
			assert_eq!(result.unwrap(), 1);
		}
	}
	assert_eq!(output, vec!(97, 98, 99));
}