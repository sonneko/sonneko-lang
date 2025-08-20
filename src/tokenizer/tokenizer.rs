use crate::tokenizer::Token;
use crate::tokenizer::error::TokenizeErr;

#[derive(Debug)]
pub struct Tokenizer {
    program: String,
    output_tokens: Vec<Token>,
    now_index: usize
}

type Result<'a> = std::result::Result<&'a Vec<Token>, TokenizeErr>;

impl Tokenizer {
    pub fn new(program: &str) -> Self {
        Tokenizer {
            program: program.into(),
            output_tokens:  Vec::new(),
            now_index: 0,
        }
    }

    fn check_next(&mut self, expect: &str) -> bool {
        let finish_index = self.now_index + expect.len();
        if finish_index > self.program.len() { return false };
        let next = &self.program[self.now_index..finish_index];
        let ret = next == expect;
        if ret {
            self.now_index += expect.len();
        }
        ret
    }

    fn push(&mut self, token: Token) {
        self.output_tokens.push(token);
    }

    fn next(&self) -> char {
        self.program.get(self.now_index..self.now_index + 1).unwrap().chars().nth(0).unwrap()
    }

    pub fn tokenize(&mut self) -> Result {
        loop {
            if self.program.len() == self.now_index { self.push(Token::EOF); break }
            else if self.check_next(" ") {}
            else if self.check_next("\t") {}
            else if self.check_next("\n") {}
            else if self.check_next(",") { self.push(Token::Comma) }
            else if self.check_next(".") { self.push(Token::Period) }
            else if self.check_next("if") { self.push(Token::If) }
            else if self.check_next("else") { self.push(Token::Else) }
            else if self.check_next("import") { self.push(Token::Import) }
            else if self.check_next("in") { self.push(Token::In) }
            else if self.check_next("from") { self.push(Token::From)  }
            else if self.check_next("static") { self.push(Token::Static) }
            else if self.check_next(":") { self.push(Token::Colon) }
            else if self.check_next(";") { self.push(Token::SemiColon) }
            else if self.check_next("const") { self.push(Token::Const) }
            else if self.check_next("let") { self.push(Token::Let) }
            else if self.check_next("fn") { self.push(Token::Fn) }
            else if self.check_next("for") { self.push(Token::For) }
            else if self.check_next("in") { self.push(Token::In) }
            else if self.check_next("let") { self.push(Token::Let) }
            else if self.check_next("return") { self.push(Token::Return) }
            else if self.check_next("(") { self.push(Token::BrancketStart) }
            else if self.check_next(")") { self.push(Token::BrancketEnd) }
            else if self.check_next("[") { self.push(Token::SquareBracketStart) }
            else if self.check_next("]") { self.push(Token::SquareBracketEnd) }
            else if self.check_next("{") { self.push(Token::CurlyBracketStart) }
            else if self.check_next("}") { self.push(Token::CurlyBracketEnd) }
            else if self.check_next("=") { self.push(Token::EqualOpe) }
            else if self.check_next("+") { self.push(Token::PlusOpe) }
            else if self.check_next("-") { self.push(Token::MinusOpe) }
            else if self.check_next("*") { self.push(Token::MulOpe) }
            else if self.check_next("/") { self.push(Token::DivOpe) }
            else if self.check_next("<") { self.push(Token::GreaterOpe) }
            else if self.check_next(">") { self.push(Token::LesserOpe) }
            else if self.check_next("true") { self.push(Token::True) }
            else if self.check_next("false") { self.push(Token::False) }
            else {
                match self.next() {
                    '0'..='9' => self.tokenize_number(),
                    '"' => self.tokenize_string(),
                    _ => self.tokenize_identifier(),
                }
            }
        }

        Ok(&self.output_tokens)

    }

    fn tokenize_number(&mut self) {
        let mut ret = Vec::<char>::new();
        while let '0'..='9' | '.' = self.next() {
            ret.push(self.next());
            self.now_index += 1;
        }
        match ret.iter().collect::<String>().parse::<i32>() {
            Ok(interger) => self.push(Token::Integer(interger)),
            Err(_) => {
                match ret.into_iter().collect::<String>().parse::<f32>() {
                    Ok(float) => self.push(Token::Float(float)),
                    Err(_) => {
                        panic!("error in parse float literal");
                    }
                }
            }
        }
    }

    fn tokenize_string(&mut self) {
        self.now_index += 1;
        let mut ret = Vec::<char>::new();
        while self.next() != '"' {
            ret.push(self.next());
            self.now_index += 1;
        }
        self.now_index += 1;
        self.push(Token::StringLiteral(ret.into_iter().collect()));
    }

    fn tokenize_identifier(&mut self) {
        let mut identifier_name = Vec::<char>::new();
        while let 'a'..='z' | 'A'..='Z' | '_' = self.next() {
            identifier_name.push(self.next());
            self.now_index += 1;
        }
        self.push(Token::Identifier(identifier_name.into_iter().collect()));
    }
}
