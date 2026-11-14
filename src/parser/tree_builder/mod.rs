pub mod block;
mod expression;
mod html;
mod postfix_processor;

use super::tokenizer::token::{Token, TokenType};
use super::tree_builder::block::AttrBuilder;
use block::{Block, BlockBuilder};

pub struct TreeBuilder<'a> {
    blocks: Vec<Block>,
    block_builders: Vec<BlockBuilder>,
    attr_builder: Option<AttrBuilder<'a>>,
}

impl<'a> TreeBuilder<'a> {
    pub fn new() -> TreeBuilder<'a> {
        TreeBuilder {
            blocks: vec![],
            block_builders: vec![],
            attr_builder: None,
        }
    }

    pub fn build(&mut self, tokens: &'a Vec<Token>) -> Vec<Block> {
        for token in tokens {
            match token.r#type() {
                TokenType::BlockIdentifier(id) => {
                    self.block_builders.push(BlockBuilder::new(id.to_owned()));
                }
                TokenType::LineComment(_) => {}
                TokenType::BlockComment(_) => {}
                TokenType::BlockOpen => {}
                TokenType::BlockClose => {
                    let mut block_builder = self.block_builders.pop().expect("Should be set.");

                    if self.attr_builder.is_some() {
                        let mut builder = self.attr_builder.take().expect("Should be set.");
                        block_builder.add_attr(builder.build());
                    }

                    match self.block_builders.last_mut() {
                        None => self.blocks.push(block_builder.build()),
                        Some(parent) => {
                            parent.add_child(block_builder);
                        }
                    }
                }
                TokenType::AttrIdentifier(id) => {
                    if self.attr_builder.is_some() {
                        panic!("Attr builder should have been built and reset at this point.")
                    }

                    self.attr_builder = Some(AttrBuilder::new(id.to_owned()));
                }
                TokenType::AttrColon => {}
                TokenType::AttrSeparator => {
                    let mut block_builder = self.block_builders.pop().expect("Should be set.");
                    let mut builder = self.attr_builder.take().expect("Should be set.");
                    block_builder.add_attr(builder.build());
                    self.block_builders.push(block_builder);
                }
                TokenType::ExprToken(expr_tok) => {
                    self.attr_builder
                        .as_mut()
                        .expect("Should be set.")
                        .expr_processor
                        .process(expr_tok);
                }
                TokenType::DirectiveOpen => {}
                TokenType::DirectiveClose => {}
                TokenType::DirectiveIdentifier(id) => {}
                TokenType::DirectiveColon => {}
                TokenType::DirectiveValue(str) => {}
                TokenType::EventListenerOpen => {}
                TokenType::EventListenerClose => {}
                TokenType::EventListenerIdentifier(id) => {}
                TokenType::EventListenerColon => {}
                TokenType::EventListenerHandler(str) => {}
            }
        }

        std::mem::take(&mut self.blocks)
    }
}
