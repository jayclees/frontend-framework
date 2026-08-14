pub mod block;
mod html;
mod expression;

use super::tokenizer::token::{Token, TokenType};
use super::tree_builder::block::{AttrBuilder, BlockContent};
use block::{Block, BlockBuilder};
use expression::Expression;

pub struct TreeBuilder {
    state_history: Vec<State>,
}

impl TreeBuilder {
    pub fn new() -> TreeBuilder {
        TreeBuilder {
            state_history: vec![State::Start],
        }
    }

    fn state(&mut self) -> &State {
        self.state_history
            .last()
            .expect("At least one should be set.")
    }

    fn push_state(&mut self, state: State) {
        self.state_history.push(state);
    }

    pub fn build(&mut self, tokens: &Vec<Token>) -> Vec<Block> {
        let mut blocks = vec![];
        let mut block_builders: Vec<BlockBuilder> = vec![];
        let mut attr_builders: Vec<AttrBuilder> = vec![];
        let mut expression: Expression = Expression::new();

        for token in tokens {
            match self.state() {
                State::Start => {
                    if let TokenType::BlockIdentifier(id) = token.r#type() {
                        block_builders.push(BlockBuilder::new(id.clone()));
                        self.push_state(State::BuildingBlock);
                    } else {
                        panic!("Invalid token at position.")
                    }
                }
                State::BuildingBlock => {
                    if token.r#type() == &TokenType::BlockClose {
                        // if root block, add to blocks vec
                        blocks.push(
                            active_block(&mut block_builders)
                                .expect("Block builder should be set.")
                                .build(),
                        )
                        // else find parent block and add as child
                    } else if let TokenType::AttrIdentifier(id) = token.r#type() {
                        // There will be reserved attr identifier keywords: text, html
                        match id.as_str() {
                            "text" => self.push_state(State::BuildingTextNode),
                            "html" => self.push_state(State::BuildingHtmlNode),
                            _ => {
                                attr_builders.push(AttrBuilder::new(id.clone()));
                                self.push_state(State::BuildingAttr);
                            }
                        }
                    }
                }
                State::BuildingTextNode => {
                    // expect expressions
                    if let TokenType::ExprToken(tok) = token.r#type() {
                        expression.push_token(tok);
                    } else {
                        match expression.is_empty() {
                            true => panic!("Expected expression."),
                            false => {
                                let block_builder =
                                    active_block(&mut block_builders).expect("Should be set");

                                // block_builder.add_content(BlockContent::Text());
                                self.state_history.pop();
                            }
                        }
                    }
                }
                State::BuildingHtmlNode => {
                    // expect expressions
                }
                State::BuildingAttr => {
                    let attr_builder =
                        active_attr(&mut attr_builders).expect("Should have attr builder.");
                    if let TokenType::ExprToken(expr) = token.r#type() {
                        attr_builder.append_expr(expr.clone());
                    } else if matches!(
                        token.r#type(),
                        TokenType::AttrSeparator | TokenType::BlockClose
                    ) {
                        let block_builder =
                            active_block(&mut block_builders).expect("Should have block builder.");
                        block_builder.add_attr(attr_builder.build());
                        attr_builders.pop();
                        self.state_history.pop();
                        if token.r#type() == &TokenType::BlockClose {
                            blocks.push(block_builder.build());
                            self.state_history.pop();
                        }
                    }
                }
                State::ExpectAttrExpression => {}
            }
        }

        fn active_block(builders: &mut [BlockBuilder]) -> Option<&mut BlockBuilder> {
            builders.last_mut()
        }

        fn active_attr(builders: &mut [AttrBuilder]) -> Option<&mut AttrBuilder> {
            builders.last_mut()
        }

        blocks
    }
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Start,
    BuildingBlock,
    BuildingAttr,
    BuildingTextNode,
    BuildingHtmlNode,
    ExpectAttrExpression,
}
