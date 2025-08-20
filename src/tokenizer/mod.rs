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
    Pub,                   // pub keyword
    NameSpace,             // namespace keyword
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
    IntegerLiteral(i32),   // 20
    FloatLiteral(f32),     // 3.2
    StringLiteral(String), // "string"
    True,                  // true
    False,                 // false
    Int,                   // int type
    Float,                 // float type
    Char,                  // char type
    Bool,                  // bool type
    EOF,                   // End of file
}
