pub mod tokenizer;
pub mod error;
mod test;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),    // my_valiable
    If,                    // if keyword
    Else,                  // else keyword
    Import,                // import keyword
    From,                  // from keyword
    Return,                // return keyword
    Static,                // static keyword
    Colon,                 // : keyword
    SemiColon,             // ; keyword
    Period,                // . keyword
    Comma,                 // , keyword
    Const,                 // const keyword
    Let,                   // let keyword
    Fn,                    // fn keyword
    For,                   // for keyword
    In,                    // in keyword
    BrancketStart,         // (
    BrancketEnd,           // )
    CurlyBracketStart,     // {
    CurlyBracketEnd,       // }
    SquareBracketStart,    // [
    SquareBracketEnd,      // ]
    EqualOpe,              // =
    PlusOpe,               // +
    MinusOpe,              // -
    MulOpe,                // *
    DivOpe,                // /
    GreaterOpe,            // <
    LesserOpe,             // >
    Integer(i32),          // 20
    Float(f32),            // 3.2
    StringLiteral(String), // "string"
    True,                  // true
    False,                 // false
    EOF,                   // End of file
}
