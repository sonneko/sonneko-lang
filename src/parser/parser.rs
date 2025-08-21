use crate::{parser::Program, tokenizer::Token, parser::error::ParseErr};

pub struct Parser {
    input: Vec<Token>,
    now_index: usize,
}

type ParseResult<T> = Result<T, ParseErr>;

impl Parser {

    fn next(&self) -> Option<&Token> {
        self.input.get(self.now_index)
    }

    fn parse(&mut self) -> Program {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Program {
        while let Ok(()) = self.parse_declaration() {

        }


        Program { declarations: Vec::new() }
    }

    fn parse_declaration(&mut self) -> ParseResult<> {
        unimplemented!()
    }

}