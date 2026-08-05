mod helper;
mod parser;

use std::fs::read_to_string;
use crate::parser::tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize(read_to_string("app/my-page.app").unwrap());
    // dbg!(tokens);
}
