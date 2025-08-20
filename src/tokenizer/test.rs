
#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenizer::Tokenizer;
    use crate::tokenizer::Token;

    #[test]
    fn test_tokenize_empty() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(*tokens, vec![Token::EOF]);
    }

    #[test]
    fn test_tokenize_whitespace() {
        let mut tokenizer = Tokenizer::new("   \t\n");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(*tokens, vec![Token::EOF]);
    }

    #[test]
    fn test_tokenize_keywords() {
        let mut tokenizer = Tokenizer::new("if import in from static const let fn for true false");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            *tokens,
            vec![
                Token::If,
                Token::Import,
                Token::In,
                Token::From,
                Token::Static,
                Token::Const,
                Token::Let,
                Token::Fn,
                Token::For,
                Token::True,
                Token::False,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_tokenize_symbols() {
        let mut tokenizer = Tokenizer::new(":;(){}[]=");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            *tokens,
            vec![
                Token::Colon,
                Token::SemiColon,
                Token::BrancketStart,
                Token::BrancketEnd,
                Token::CurlyBracketStart,
                Token::CurlyBracketEnd,
                Token::SquareBracketStart,
                Token::SquareBracketEnd,
                Token::EqualOpe,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_tokenize_operators() {
        let mut tokenizer = Tokenizer::new("+-*/");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            *tokens,
            vec![
                Token::PlusOpe,
                Token::MinusOpe,
                Token::MulOpe,
                Token::DivOpe,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_tokenize_mixed() {
        let mut tokenizer = Tokenizer::new("let=\"hello world\"2232 43.43; if (true)");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            *tokens,
            vec![
                Token::Let,
                Token::EqualOpe,
                Token::StringLiteral("hello world".into()),
                Token::Integer(2232),
                Token::Float(43.43),
                Token::SemiColon,
                Token::If,
                Token::BrancketStart,
                Token::True,
                Token::BrancketEnd,
                Token::EOF,
            ]
        );
    }


    #[test]
    fn test_tokenize_complex() {
        let mut tokenizer = Tokenizer::new(
            r#"
let x = 10;
const PI = 3.14;
fn add(a, b) {
    if a > 0 {
        return a + b;
    } else {
        return 0;
    }
}
import { util } from "module";
for i in [1, 2, 3] {
    static count = 0;
    count = count + 1;
}
"#,
        );
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            *tokens,
            vec![
                Token::Let,
                Token::Identifier("x".into()),
                Token::EqualOpe,
                Token::Integer(10),
                Token::SemiColon,
                Token::Const,
                Token::Identifier("PI".into()),
                Token::EqualOpe,
                Token::Float(3.14),
                Token::SemiColon,
                Token::Fn,
                Token::Identifier("add".into()),
                Token::BrancketStart,
                Token::Identifier("a".into()),
                Token::Comma,
                Token::Identifier("b".into()),
                Token::BrancketEnd,
                Token::CurlyBracketStart,
                Token::If,
                Token::Identifier("a".into()),
                Token::LesserOpe,
                Token::Integer(0),
                Token::CurlyBracketStart,
                Token::Return,
                Token::Identifier("a".into()),       
                Token::PlusOpe,
                Token::Identifier("b".into()),
                Token::SemiColon,
                Token::CurlyBracketEnd,
                Token::Else,
                Token::CurlyBracketStart,
                Token::Return,
                Token::Integer(0),
                Token::SemiColon,
                Token::CurlyBracketEnd,
                Token::CurlyBracketEnd,
                Token::Import,
                Token::CurlyBracketStart,
                Token::Identifier("util".into()),
                Token::CurlyBracketEnd,
                Token::From,
                Token::StringLiteral("module".into()),
                Token::SemiColon,
                Token::For,
                Token::Identifier("i".into()),
                Token::In,
                Token::SquareBracketStart,
                Token::Integer(1),
                Token::Comma,
                Token::Integer(2),
                Token::Comma,
                Token::Integer(3),
                Token::SquareBracketEnd,
                Token::CurlyBracketStart,
                Token::Static,
                Token::Identifier("count".into()),
                Token::EqualOpe,
                Token::Integer(0),
                Token::SemiColon,
                Token::Identifier("count".into()),
                Token::EqualOpe,
                Token::Identifier("count".into()),
                Token::PlusOpe,
                Token::Integer(1),
                Token::SemiColon,
                Token::CurlyBracketEnd,
                Token::EOF,
            ]
        );
    }

}
