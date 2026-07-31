mod helper;
mod parser;

use crate::parser::tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize();
    dbg!(tokens);
}
