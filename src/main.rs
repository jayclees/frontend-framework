mod helper;
mod parser;
mod generator;

use std::fs::read_to_string;
use std::path::PathBuf;
use crate::generator::Generator;
use crate::parser::tokenizer::Tokenizer;
use crate::parser::tree_builder::TreeBuilder;

fn main() {
    let mut tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize(read_to_string("app/counter.app").unwrap());
    let mut builder = TreeBuilder::new();
    let tree = builder.build(tokens);
    let mut generator = Generator::new(&tree, PathBuf::from("dist"));
    dbg!(generator.generate());
}
