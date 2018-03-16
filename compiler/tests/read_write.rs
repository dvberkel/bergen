use std::io::{Read};

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