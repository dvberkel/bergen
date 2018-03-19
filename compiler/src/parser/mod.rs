use super::brnfck::Command;

pub fn parse(_source: &[u8]) -> Result<Vec<Command>, ParseError> {
    Ok(vec![])
}

pub enum ParseError {
    Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_empty_source() {
        let source: &[u8] = "".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 0);
        } else {
            assert!(false);
        }
    }
}
