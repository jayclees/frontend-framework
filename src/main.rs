mod helper;
mod parser;
mod generator;

use std::fs::read_to_string;
use std::path::PathBuf;
use serde_json::json;
use crate::generator::Generator;
use crate::parser::tokenizer::Tokenizer;
use crate::parser::tree_builder::TreeBuilder;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(read_to_string("test-stubs/basic.app").unwrap());
    let mut builder = TreeBuilder::new();
    let mut tree = builder.build(tokens);
    let mut generator = Generator::new(&mut tree, PathBuf::from("dist/target.html"));
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

    // todo: layouts support, [extends: "my-layout"], @include("my-partial")
    // todo: create command line interface for parser/generator
}
