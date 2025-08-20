use std::fmt;

pub enum TokenizeErr {
    UnexpectedChar(char, usize),
    UnexpectedEOF(usize),
}

impl fmt::Debug for TokenizeErr {
    fn fmt (&self, _formatter: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        println!("failed to tokenize program because...");
        match self {
            Self::UnexpectedChar(target, position) => println!("there is unexpected character: {} in {}", target, position),
            Self::UnexpectedEOF(position) => println!("there is unexpexted EOF in {}", position),
        }

        Ok(())
    }
}
