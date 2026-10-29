pub mod block;
mod expression;
mod expr_parser;
mod html;

use super::tokenizer::token::{ExprToken, Token, TokenType};
use super::tree_builder::block::{AttrBuilder, BlockContent};
use block::{Block, BlockBuilder};

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

    fn pop_state(&mut self, state: Option<State>) {
        match state {
            Some(state) => {
                if *self.state_history.last().expect("Should be set.") == state {
                    self.state_history.pop();
                } else {
                    panic!("State attempting to be popped is not current one.")
                }
            }
            None => {
                self.state_history.pop();
            }
        }
    }

    pub fn build(&mut self, tokens: &Vec<Token>) -> Vec<Block> {
        let mut blocks = vec![];
        let mut block_builders: Vec<BlockBuilder> = vec![];
        let mut attr_builder: Option<AttrBuilder> = None;
        let mut expression: Vec<&ExprToken> = vec![];

        // dbg!(&tokens);

        for token in tokens {
            match self.state() {
                State::Start => match token.r#type() {
                    TokenType::BlockIdentifier(id) => {
                        block_builders.push(BlockBuilder::new(id.clone()));
                        self.push_state(State::BuildingBlock);
                    }
                    _ => panic!("Invalid token at position."),
                },
                State::BuildingBlock => {
                    if token.r#type() == &TokenType::BlockClose {
                        // if root block, add to blocks vec
                        blocks.push(build_active_block(&mut block_builders))
                        // else find parent block and add as child
                    } else if let TokenType::AttrIdentifier(id) = token.r#type() {
                        // There will be reserved attr identifier keywords: text, html
                        match id.as_str() {
                            "text" => self.push_state(State::BuildingTextNode),
                            "html" => self.push_state(State::BuildingHtmlNode),
                            // todo handle other specific attrs
                            _ => {
                                attr_builder = Some(AttrBuilder::new(id.clone()));
                                self.push_state(State::BuildingAttr);
                            }
                        }
                    }
                }
                State::BuildingTextNode => {
                    match token.r#type() {
                        TokenType::ExprToken(tok) => {
                            expression.push(tok);
                        }
                        TokenType::AttrSeparator | TokenType::BlockClose => {
                            if expression.is_empty() {
                                // dbg!(tokens, token);
                                panic!("Expected expression.");
                            } else {
                                let block_builder = active_block(&mut block_builders);
                                // block_builder.add_content(expression.resolve());
                                block_builder.add_content(BlockContent::Text("foobar".to_owned()));
                                blocks.push(build_active_block(&mut block_builders));
                                self.pop_state(None);
                            }
                        }
                        _ => {
                            if let TokenType::AttrColon = token.r#type() {
                                continue;
                            }
                            dbg!(token);
                            unimplemented!()
                        }
                    }
                }
                State::BuildingHtmlNode => {
                    // expect expressions
                }
                State::BuildingAttr => {
                    let builder = attr_builder.as_mut().unwrap();
                    if let TokenType::ExprToken(expr) = token.r#type() {
                        builder.add_expr_token(&expr);
                    } else if matches!(
                        token.r#type(),
                        TokenType::AttrSeparator | TokenType::BlockClose
                    ) {
                        let block_builder = active_block(&mut block_builders);
                        block_builder.add_attr(builder.build());
                        attr_builder = None;
                        self.pop_state(None);
                        if token.r#type() == &TokenType::BlockClose {
                            blocks.push(block_builder.build());
                            self.pop_state(Some(State::BuildingBlock));
                        }
                    }
                }
                State::ExpectAttrExpression => {}
            }
        }

        fn active_block(builders: &mut [BlockBuilder]) -> &mut BlockBuilder {
            builders.last_mut().expect("Should be set")
        }

        fn build_active_block(builders: &mut Vec<BlockBuilder>) -> Block {
            let block = active_block(builders).build();
            builders.pop();
            block
        }

        // fn active_attr(builders: &mut [AttrBuilder]) -> Option<&mut AttrBuilder> {
        //     builders.last_mut()
        // }

        dbg!(&blocks);

        blocks
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Start,
    BuildingBlock,
    BuildingAttr,
    BuildingTextNode,
    BuildingHtmlNode,
    ExpectAttrExpression,
}
