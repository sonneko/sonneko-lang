use crate::{parser::Program, tokenizer::Token};

pub struct Parser {
    input: Vec<Token>,
    now_index: usize,
}

impl Parser {

    fn next(&self) -> Token {
        *self.input.get(self.now_index..self.now_index + 1).unwrap().first().unwrap()
    }

    fn expect(&mut self, token: Token) {
        assert!(self.next() == token);
        self.now_index += 1;
    }

    fn is_next(&mut self, token: Token) -> bool {
        self.next() == token
    }

    fn parse(&mut self) -> Program {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Program {
        self.
    }

}