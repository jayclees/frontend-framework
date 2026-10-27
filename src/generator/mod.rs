use crate::parser::tree_builder::block::Block;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct Generator<'a> {
    blocks: &'a Vec<Block>,
    target_dir: PathBuf,
}

impl<'a> Generator<'a> {
    pub fn new(blocks: &'a Vec<Block>, target_dir: PathBuf) -> Generator<'a> {
        Generator { blocks, target_dir }
    }

    pub fn generate(&mut self) {
        let mut html = String::new();
        for block in self.blocks {
            html.push_str(block.to_html().as_str());
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
                println!("{written} bytes written to {}", self.target_dir.to_str().unwrap());
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
        // convert tree into html
        // if block is page, check app root, generate js along with it
    }
}
