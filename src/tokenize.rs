

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i32),   // example: 3
    Float(f32),     // example: 9.8
    AddOpe,         // +
    SubOpe,         // -
    MulOpe,         // *
    DivOpe,         // /
}

#[derive(Debug, PartialEq)]
pub enum TokenizeErrType {
    IllegalFloatLiteral,
    UnknownToken
}

#[derive(Debug)]
pub struct TokenizeErr {
    err_type: TokenizeErrType,
    location: usize
}

#[derive(Debug)]
pub struct Tokenizer{
    source_str: String,
    now_index: usize
}

impl Tokenizer {

    /// constructor
    pub fn new(source_str: &str) -> Self {
        Self {
            source_str: source_str.to_string(),
            now_index: 0
        }
    }

    /// entry point
    pub fn tokenize(mut self) -> Result<Vec<Token>, TokenizeErr> {
        let mut ret: Vec<Token> = Vec::new();

        while self.source_str.len() > self.now_index {
            if self.is_starting_with(" ") {
                self.step(1);
                continue;
            } else if self.is_starting_with("+") {
                self.step(1);
                ret.push(Token::AddOpe);
                continue;
            } else if self.is_starting_with("*") {
                self.step(1);
                ret.push(Token::MulOpe);
                continue;
            } else if self.is_starting_with("/") {
                self.step(1);
                ret.push(Token::DivOpe);
                continue;
            } else if self.is_starting_with("-") {
                self.step(1);
                ret.push(Token::SubOpe);
                continue;
            } else if let Some(float_num) = self.float() {
                ret.push(Token::Float(float_num));
                continue;
            } else if let Some(integer_num) = self.integer() {
                ret.push(Token::Integer(integer_num));
                continue;
            } else {
                return Err(TokenizeErr {
                    location: self.now_index,
                    err_type: TokenizeErrType::UnknownToken
                })
            }
        }

        Ok(ret)
    }

    /// helper fn
    fn next_str(&self, len: usize) -> Option<&str> {
        let start_index = self.now_index;
        let end_index = self.now_index + len;
        self.source_str.get(start_index..end_index)
    }

    /// helper fn
    fn next_char(&self) -> Option<char>{
        self.next_str(1).and_then(|target_str| target_str.chars().nth(0))
    }

    /// helper fn
    fn prev_str(&self, len: usize) -> Option<&str> {
        let start_index = self.now_index - len;
        let end_index = self.now_index;
        self.source_str.get(start_index..end_index)
    }

    /// helper fn
    fn is_starting_with(&self, s: &str) -> bool{
        if let Some(target) = self.next_str(s.len()) {
            return target == s;
        }
        false
    }
    
    /// helper fn
    fn step(&mut self, amount: usize) {
        assert!(self.now_index + amount <= self.source_str.len(), "step method overflow! {} can't step; {:?}", amount, self);
        self.now_index += amount;
    }

    /// helper fn
    fn back(&mut self, amount: usize) {
        assert!(self.now_index - amount >= 0, "back method overflow! {} can't back; {:?}", amount, self);
        self.now_index -= amount;
    }

    /// tokenize integer literal
    fn integer(&mut self) -> Option<i32> {
        let temp_index = self.now_index;
        while let Some('0'..='9') = self.next_char() {
            self.step(1);
        }
        let number_length = self.now_index - temp_index;
        if number_length == 0 {
            return None;
        }
        let integer: Result<i32, _>=  self.prev_str(number_length).unwrap().parse();
        match integer {
            Ok(value) => Some(value),
            Err(_) => {
                self.back(number_length);
                None
            }
        }
    }

    /// tokenize float literal
    fn float(&mut self) -> Option<f32> {
        let temp_index = self.now_index;
        while let Some('0'..='9' | '.') = self.next_char() {
            self.step(1);
        }
        let number_length = self.now_index - temp_index;
        let target_str = self.prev_str(number_length).unwrap();
        
        match target_str.chars().filter(|i| *i == '.').count() {
            0 => {
                self.back(number_length);
                None
            },
            1 => Some(target_str.parse().unwrap()),
            2.. => {
                self.back(number_length);
                None
            }
        }
    }

}



#[cfg(test)]
mod test {
    use super::*;

    struct TestExample {
        source_code: String,
        expect_tokens: Vec<Token>
    }

    fn test_tokenize(example: TestExample) {
        let tokenizer = Tokenizer::new(&example.source_code);
        let expect = example.expect_tokens;
        assert_eq!(tokenizer.tokenize().unwrap(), expect);
    }

    #[test]
    fn tokenize_one_integer() {
        test_tokenize(TestExample { 
            source_code: "10".to_string(),
            expect_tokens: vec![Token::Integer(10)]
        });
    }

    #[test]
    fn tokenize_one_float() {
        test_tokenize(TestExample {
            source_code: "1.3".to_string(),
            expect_tokens: vec![Token::Float(1.3)]
        });
    }
    
    #[test]
    fn tokenize_add_ope() {
        test_tokenize(TestExample {
            source_code: "10+ 23".to_string(),
            expect_tokens: vec![Token::Integer(10), Token::AddOpe, Token::Integer(23)]
        });
    }

    #[test]
    fn tokenize_sub_ope() {
        test_tokenize(TestExample {
            source_code: " 10 +2.0-17 ".to_string(),
            expect_tokens: vec![Token::Integer(10), Token::AddOpe, Token::Float(2.0), Token::SubOpe, Token::Integer(17)]
        });
    }

    #[test]
    fn tokenize_complex_sub_and_add_expresion() {
        test_tokenize(TestExample {
            source_code: "10 +.3- 3.8 22+2.".to_string(),
            expect_tokens: vec![Token::Integer(10), Token::AddOpe, Token::Float(0.3), Token::SubOpe, Token::Float(3.8), Token::Integer(22), Token::AddOpe, Token::Float(2.0)]
        });
    }

    #[test]
    fn tokenize_complex_sub_add_mul_div_expresion() {
        test_tokenize(TestExample {
            source_code: "10* .48 /3.2* 4. .5".to_string(),
            expect_tokens: vec![Token::Integer(10), Token::MulOpe, Token::Float(0.48), Token::DivOpe, Token::Float(3.2), Token::MulOpe, Token::Float(4.0), Token::Float(0.5)]
        });
    }

}
