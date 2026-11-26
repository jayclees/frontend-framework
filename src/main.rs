mod generator;
mod helper;
mod parser;

use crate::generator::Generator;
use crate::parser::tokenizer::Tokenizer;
use crate::parser::tree_builder::TreeBuilder;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(read_to_string("test-stubs/basic.app").unwrap());
    let mut builder = TreeBuilder::new();
    // todo ensure state is an object
    let state = serde_json::from_str(
        read_to_string("test-stubs/test-data.json")
            .unwrap()
            .as_ref(),
    ).unwrap();
    let mut tree = builder.build(tokens, &state);
    let mut generator = Generator::new(&mut tree, PathBuf::from("dist/target.html"));
    generator.generate(&state);
}
