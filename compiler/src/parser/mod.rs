use super::brnfck::Command;

const NEWLINE : u8 = 10u8;

pub fn parse(source: &[u8]) -> Result<Vec<Command>, ParseError> {
    rows(source).and_then(|(top, middle, bottom)|{
        if top.len() != middle.len() || middle.len() != bottom.len() { return Err(ParseError::DifferentNumberOfRows)}
        let mut program = vec![];
        let mut column = 0;
        if column < top.len() {
            program.push(Command::IncrementPointer);
        } 
        Ok(program)
    })
}


fn rows(source: &[u8]) -> Result<(&[u8], &[u8], &[u8]), ParseError> {
    let mut index = 0;
    while index < source.len() && source[index] != NEWLINE {
        index += 1;
    }
    let first_index = index;
    index += 1;
    while index < source.len() && source[index] != NEWLINE {
        index += 1;
    }
    let second_index = index;
    index += 1;
    while index < source.len() && source[index] != NEWLINE {
        index += 1;
    }
    let third_index = index;
    if index < source.len() {
        Ok((&source[             0 .. first_index],
            &source[ first_index+1 .. second_index],
            &source[second_index+1 .. third_index]))
    } else {
        Err(ParseError::NotEnoughRows)
    }
}
#[derive(Debug, PartialEq)]
pub enum ParseError {
    Unknown,
    DifferentNumberOfRows,
    NotEnoughRows,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::brnfck::Command;

    #[test]
    fn should_parse_empty_source() {
        let source: &[u8] = "\n\n\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_parse_increment_pointer() {
        let source: &[u8] = "  /\\  \n /  \\ \n/    \\\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions, vec![Command::IncrementPointer])
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_throw_when_number_of_columns_do_not_agree() {
        let source: &[u8] = " \n\n\n".as_bytes();

        if let Err(problem) = parse(source) {
            assert_eq!(problem, ParseError::DifferentNumberOfRows);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_throw_when_there_are_to_few_rows() {
        let source: &[u8] = " \n\n".as_bytes();

        if let Err(problem) = parse(source) {
            assert_eq!(problem, ParseError::NotEnoughRows);
        } else {
            assert!(false);
        }
    }
}