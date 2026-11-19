mod generator;
mod helper;
mod parser;
use crate::generator::Generator;
use crate::parser::tokenizer::Tokenizer;
use crate::parser::tree_builder::TreeBuilder;
use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;
use serde_json::json;

fn main() {
    let args = Args::parse();

    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(read_to_string(args.input_file).unwrap());
    let mut builder = TreeBuilder::new();
    let tree = builder.build(tokens);
    let mut generator = Generator::new(&tree, PathBuf::from(args.output_dir));
    generator.generate(json!({
        "posts": [
            {
                "id": 1,
                "title": "Hello world!",
                "body": "This is the first post.",
                "published": "01-01-1970",
            },
            {
                "id": 1,
                "title": "Foo bar",
                "body": "Baz.",
                "published": "01-02-1970",
            },
        ]
    }));
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the source file
    #[arg(short, long)]
    input_file: String,

    /// Output path
    #[arg(short, long)]
    output_dir: String,
}
