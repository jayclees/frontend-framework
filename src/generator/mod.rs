use crate::parser::tree_builder::block::Block;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use serde_json::Value;
use crate::parser::tokenizer::directive::DirectiveEnum;

pub struct Generator<'token> {
    blocks: &'token mut Vec<Block<'token>>,
    target_dir: PathBuf,
}

impl<'token> Generator<'token> {
    pub fn new(blocks: &'token mut Vec<Block<'token>>, target_dir: PathBuf) -> Self {
        Generator { blocks, target_dir }
    }

    pub fn generate(&mut self, state: Value) {
        let mut html = String::from("<!doctype html>");
        for block in self.blocks.iter_mut() {
            html.push_str(block.render_html(&state).as_str());
        }
        // todo use rayon for certain tasks?
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.target_dir.clone())
            .unwrap();

        match file.write(html.as_bytes()) {
            Ok(written) => {
                println!("{html}");
                println!(
                    "{written} bytes written to {}",
                    self.target_dir.to_str().unwrap()
                );
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
        // convert tree into html
        // if block is page, check app root, generate js along with it
    }
}
