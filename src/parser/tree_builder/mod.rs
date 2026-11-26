pub mod block;
pub(crate) mod postfix_processor;

use super::tokenizer::token::{Token, TokenType};
use super::tree_builder::block::AttrBuilder;
use crate::parser::tokenizer::directive::{Directive, DirectiveEnum, ForDirectiveBuildState};
use block::{Block, BlockBuilder};
use serde_json::{Number, Value};

pub struct TreeBuilder<'token> {
    blocks: Vec<Block<'token>>,
    block_builders: Vec<BlockBuilder<'token>>,
    attr_builder: Option<AttrBuilder<'token>>,
    active_directive: Option<DirectiveEnum<'token>>,
}

impl<'token> TreeBuilder<'token> {
    pub fn new() -> TreeBuilder<'token> {
        TreeBuilder {
            blocks: vec![],
            block_builders: vec![],
            attr_builder: None,
            active_directive: None,
        }
    }

    pub fn build(&mut self, tokens: &'token Vec<Token>, state: &Value) -> Vec<Block<'token>> {
        for token in tokens {
            match token.r#type() {
                TokenType::BlockIdentifier(id) => {
                    self.block_builders.push(BlockBuilder::new(id.to_owned()))
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
                            .as_mut()
                            .unwrap()
                            .add(expr_tok);
                    }
                }

                // Directives...
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

                // For directive...
                TokenType::ForDirectiveParenthesesOpen => {
                    if let Some(DirectiveEnum::For(directive)) = self.active_directive.as_mut() {
                        directive.set_state(ForDirectiveBuildState::ReceiveIndexVar);
                    } else {
                        panic!("Active directive should be set.")
                    }
                }
                TokenType::ForDirectiveParenthesesClose => {}
                TokenType::ForDirectiveInKeyword => {
                    if let Some(DirectiveEnum::For(directive)) = self.active_directive.as_mut() {
                        directive.set_state(ForDirectiveBuildState::ReceiveIterExpr);
                    } else {
                        panic!("Active directive should be set.")
                    }
                }
                TokenType::ForDirectiveIndexComma => {
                    if let Some(DirectiveEnum::For(directive)) = self.active_directive.as_mut() {
                        directive.set_state(ForDirectiveBuildState::ReceiveItemVar);
                    } else {
                        panic!("Active directive should be set.")
                    }
                }

                // Event listeners...
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

pub(super) fn value_as_string(value: Value) -> String {
    match value {
        Value::String(s) => s.to_owned(),
        Value::Number(n) => n.to_string(),
        Value::Null => "null".to_string(),
        Value::Bool(bool) => bool.to_string(),
        Value::Array(_) => unimplemented!(),
        Value::Object(_) => unimplemented!(),
    }
}

pub(super) fn value_as_bool(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(bool) => *bool,
        Value::Number(n) => !n.eq(&Number::from_u128(0).unwrap()),
        Value::String(_) => panic!("String evaluation as boolean not supported."),
        Value::Array(_) => panic!("Array evaluation as boolean not supported."),
        Value::Object(_) => panic!("Object evaluation as boolean not supported."),
    }
}