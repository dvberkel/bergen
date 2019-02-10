use super::brnfck::Command;

const NEWLINE : u8 = 10u8;

pub fn parse(source: &[u8]) -> Result<Vec<Command>, ParseError> {
    rows(source).and_then(|(top, middle, bottom)|{
        if top.len() != middle.len() || middle.len() != bottom.len() { return Err(ParseError::DifferentNumberOfRows)}
        let mut program = vec![];
        let mut column = 0;
        if column < top.len() {
            if let Some((command, next_column)) = peek(column, top, middle, bottom) {
                column = next_column;
                program.push(command);
            } else {
                return Err(ParseError::UnknownMountainRange(column));
            }
        } 
        Ok(program)
    })
}

fn peek(column: usize, top: &[u8], middle: &[u8], bottom: &[u8]) -> Option<(Command, usize)> {
    if (column + 6) <= top.len() {
        if    &top[column .. column + 6] == "  /\\  ".as_bytes() &&
           &middle[column .. column + 6] == " /  \\ ".as_bytes() &&
           &bottom[column .. column + 6] == "/    \\".as_bytes() {
               return Some((Command::IncrementPointer, column + 6));
           }
    }
    if (column + 8) <= top.len() {
        if    &top[column .. column + 8] == "  /\\/\\  ".as_bytes() &&
           &middle[column .. column + 8] == " /    \\ ".as_bytes() &&
           &bottom[column .. column + 8] == "/      \\".as_bytes() {
               return Some((Command::DecrementPointer, column + 8));
           } 
    }
    if (column + 4) <= top.len() {
        if    &top[column .. column + 4] == "    ".as_bytes() &&
           &middle[column .. column + 4] == " /\\ ".as_bytes() &&
           &bottom[column .. column + 4] == "/  \\".as_bytes() {
               return Some((Command::Increment, column + 4));
           } 
    }
    if (column + 6) <= top.len() {
        if    &top[column .. column + 6] == "      ".as_bytes() &&
           &middle[column .. column + 6] == " /\\/\\ ".as_bytes() &&
           &bottom[column .. column + 6] == "/    \\".as_bytes() {
               return Some((Command::Decrement, column + 6));
           } 
    }
    if (column + 8) <= top.len() {
        if    &top[column .. column + 8] == "  /\\    ".as_bytes() &&
           &middle[column .. column + 8] == " /  \\/\\ ".as_bytes() &&
           &bottom[column .. column + 8] == "/      \\".as_bytes() {
               return Some((Command::JumpAhead, column + 8));
           } 
    }
    if (column + 8) <= top.len() {
        if    &top[column .. column + 8] == "    /\\  ".as_bytes() &&
           &middle[column .. column + 8] == " /\\/  \\ ".as_bytes() &&
           &bottom[column .. column + 8] == "/      \\".as_bytes() {
               return Some((Command::JumpBack, column + 8));
           } 
    }
    if (column + 2) <= top.len() {
        if    &top[column .. column + 2] == "  ".as_bytes() &&
           &middle[column .. column + 2] == "  ".as_bytes() &&
           &bottom[column .. column + 2] == "/\\".as_bytes() {
               return Some((Command::Write, column + 2));
           } 
    }
    None
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
    NotEnoughRows,
    DifferentNumberOfRows,
    UnknownMountainRange(usize),
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
    fn should_parse_decrement_pointer() {
        let source: &[u8] = "  /\\/\\  \n /    \\ \n/      \\\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions, vec![Command::DecrementPointer])
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_parse_increment() {
        let source: &[u8] = "    \n /\\ \n/  \\\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions, vec![Command::Increment])
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_parse_decrement() {
        let source: &[u8] = "      \n /\\/\\ \n/    \\\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions, vec![Command::Decrement])
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_parse_jump_ahead() {
        let source: &[u8] = "  /\\    \n /  \\/\\ \n/      \\\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions, vec![Command::JumpAhead])
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_parse_jump_back() {
        let source: &[u8] = "    /\\  \n /\\/  \\ \n/      \\\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions, vec![Command::JumpBack])
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_parse_write() {
        let source: &[u8] = "  \n  \n/\\\n".as_bytes();

        if let Ok(instructions) = parse(source) {
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions, vec![Command::Write])
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