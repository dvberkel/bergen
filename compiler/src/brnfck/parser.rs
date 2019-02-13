use crate::brnfck::Command;

pub fn parse(source: &[u8]) -> Result<Vec<Command>, ParseError> {
    let mut program = Vec::new();
    let mut index = 0;
    while index < source.len() {
        match source[index] {
            43 /* + */ => program.push(Command::Increment),
            44 /* , */ => program.push(Command::Read),
            45 /* - */ => program.push(Command::Decrement),
            46 /* . */ => program.push(Command::Write),
            60 /* < */ => program.push(Command::DecrementPointer),
            62 /* > */ => program.push(Command::IncrementPointer),
            91 /* [ */ => program.push(Command::JumpAhead),
            93 /* ] */ => program.push(Command::JumpBack),
            10 /* LF */ => { /* Accept, but ignore */},
            13 /* CR */ => { /* Accept, but ignore */},
            32 /* SPACE */ => { /* Accept, but ignore */ },
            _ => return Err(ParseError::UnknownCharacter(source[index])),
        }
        index += 1;
    }
    Ok(program)
}

pub enum ParseError {
    UnknownCharacter(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_Increment() {
        let source = "+".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::Increment));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_Decrement() {
        let source = "-".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::Decrement));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_IncrementPointer() {
        let source = ">".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::IncrementPointer));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_DecrementPointer() {
        let source = "<".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::DecrementPointer));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_JumpAhead() {
        let source = "[".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::JumpAhead));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_JumpBack() {
        let source = "]".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::JumpBack));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_Write() {
        let source = ".".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::Write));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_Read() {
        let source = ",".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 1);
            assert_eq!(program, vec!(Command::Read));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_program() {
        let source = "++[-]".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 5);
            assert_eq!(program, vec!(Command::Increment, Command::Increment, Command::JumpAhead, Command::Decrement, Command::JumpBack));
        } else {
            assert!(false);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_parse_program_wuth_whitespace() {
        let source = "++ [ - ] ".as_bytes();

        if let Ok(program) = parse(&source) {
            assert_eq!(program.len(), 5);
            assert_eq!(program, vec!(Command::Increment, Command::Increment, Command::JumpAhead, Command::Decrement, Command::JumpBack));
        } else {
            assert!(false);
        }
    }
}