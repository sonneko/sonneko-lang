mod tokenizer;
mod parser;
use tokenizer::tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("");
    tokenizer.tokenize().unwrap();
    println!("Hello World!");
}