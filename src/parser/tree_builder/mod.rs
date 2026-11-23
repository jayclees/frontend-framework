pub mod block;
pub mod component;
pub(crate) mod postfix_processor;

use super::tokenizer::token::{Token, TokenType};
use super::tree_builder::block::AttrBuilder;
use crate::parser::tokenizer::directive::{Directive, DirectiveEnum};
use crate::parser::tree_builder::component::ComponentInterface;
use block::{Block, BlockBuilder};
use std::collections::HashMap;
use serde_json::Value;

pub struct TreeBuilder<'token, C: ComponentInterface> {
    blocks: Vec<Block<'token>>,
    block_builders: Vec<BlockBuilder<'token, C>>,
    attr_builder: Option<AttrBuilder<'token>>,
    active_directive: Option<DirectiveEnum<'token>>,
    component_factories: HashMap<&'static str, fn() -> C>,
}

impl<'token, C: ComponentInterface> TreeBuilder<'token, C> {
    pub fn new() -> TreeBuilder<'token, C> {
        TreeBuilder {
            blocks: vec![],
            block_builders: vec![],
            attr_builder: None,
            active_directive: None,
            component_factories: HashMap::new(),
        }
    }

    pub fn component(&mut self, str: &'static str, factory: fn() -> C) {
        self.component_factories.insert(str, factory);
    }

    pub fn build(&mut self, tokens: &'token Vec<Token>, state: &Value) -> Vec<Block<'token>> {
        for token in tokens {
            match token.r#type() {
                TokenType::BlockIdentifier(id) => {
                    let component_factory = self.component_factories.get(id.as_str());
                    match component_factory {
                        Some(factory) => {
                            self
                                .block_builders
                                .push(BlockBuilder::new(id.to_owned(), Some(factory())))
                        }
                        None => self
                            .block_builders
                            .push(BlockBuilder::new(id.to_owned(), None)),
                    }
                }
                TokenType::LineComment(_) => {}
                TokenType::BlockComment(_) => {}
                TokenType::BlockOpen => {}
                TokenType::BlockClose => {
                    let mut block_builder = self.block_builders.pop().expect("Should be set.");

                    if self.attr_builder.is_some() {
                        let mut builder = self.attr_builder.take().expect("Should be set.");
                        block_builder.add_attr(builder.build(state));
                    }

                    match self.block_builders.last_mut() {
                        None => self.blocks.push(block_builder.build()),
                        Some(parent) => parent.add_child(block_builder),
                    };
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
                    block_builder.add_attr(builder.build(state));
                    self.block_builders.push(block_builder);
                }
                TokenType::ExprToken(expr_tok) => {
                    if let Some(directive) = self.active_directive.as_mut() {
                        match directive {
                            DirectiveEnum::If(dir) => dir.add_expr_token(expr_tok),
                            DirectiveEnum::ElseIf(dir) => dir.add_expr_token(expr_tok),
                            DirectiveEnum::Else(dir) => dir.add_expr_token(expr_tok),
                            DirectiveEnum::For(dir) => dir.add_expr_token(expr_tok),
                            DirectiveEnum::Extends(dir) => dir.add_expr_token(expr_tok),
                            DirectiveEnum::DummyMarker(dir) => dir.add_expr_token(expr_tok),
                            DirectiveEnum::DummyString(dir) => dir.add_expr_token(expr_tok),
                        }
                    } else {
                        self.attr_builder
                            .as_mut()
                            .expect("Should be set.")
                            .expr_processor
                            .add(expr_tok);
                    }
                }
                TokenType::DirectiveOpen => {}
                TokenType::DirectiveClose => self
                    .block_builders
                    .last_mut()
                    .unwrap()
                    .add_directive(self.active_directive.take().unwrap()),
                TokenType::DirectiveIdentifier(id) => {
                    self.active_directive = Some(DirectiveEnum::from(id));
                }
                TokenType::DirectiveColon => {}
                TokenType::DirectiveValue(str) => {
                    // active_directive = Some(directive);
                }
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
