mod helper;
mod parser;
mod generator;

use std::fs::read_to_string;
use std::path::PathBuf;
use crate::generator::Generator;
use crate::parser::Parser;
use crate::parser::tokenizer::Tokenizer;
use crate::parser::tree_builder::TreeBuilder;

fn main() {
    let mut parser = Parser {};
    parser.run()
}
