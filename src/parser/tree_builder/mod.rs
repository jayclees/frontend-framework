mod block;
mod html;

use super::tokenizer::token::Token;
use crate::parser::tree_builder::block::AttrBuilder;
use block::{Block, BlockBuilder};

pub struct TreeBuilder {
    state_history: Vec<Option<State>>,
    active_blocks: Vec<BlockBuilder>,
}

impl TreeBuilder {
    pub fn new() -> TreeBuilder {
        TreeBuilder {
            state_history: vec![Some(State::Start)],
            active_blocks: vec![],
        }
    }

    // fn state(&mut self) -> State {
    //     self.state_history.last_mut()
    // }

    // fn active_block(&mut self) -> Option<BlockBuilder> {
    //     for state in self.state_history.iter_mut().rev() {
    //         if let State::BuildingBlock(builder) = state.unwrap() {
    //             return Some(builder);
    //         }
    //     }
    //
    //     None
    // }

    fn parent_block(&mut self) -> Option<&mut BlockBuilder> {
        self.active_blocks.iter_mut().nth_back(1)
    }

    pub fn build(&mut self, tokens: &Vec<Token>) -> Vec<Block> {
        let mut blocks = vec![];
        let mut state_history: Vec<Option<State>> = vec![Some(State::Start)];
        let active_block: Option<BlockBuilder> = None;
        let active_attr: Option<AttrBuilder> = None;

        for token in tokens {
            let (holder, state) = state(&mut state_history);
            match state {
                State::Start => {
                    // if let TokenType::BlockIdentifier(id) = token.r#type() {
                    //     let builder = BlockBuilder::new(id.clone());
                    //     let mut state_history = state_history.borrow_mut();
                    //     state_history.push(State::BuildingBlock(builder));
                    // } else {
                    //     panic!("Invalid token at position.")
                    // }
                }
                State::BuildingBlock(ref block_builder) => {}
                State::BuildingAttr(ref attr_builder) => {
                    // if let TokenType::ExprToken(expr) = token.r#type() {
                    //     attr_builder.append_expr(expr.clone());
                    // } else if matches!(token.r#type(), TokenType::AttrSeparator | TokenType::BlockClose) {
                    //     let active_block = self.active_block();
                    //     active_block.unwrap().add_attr(attr_builder.build());
                    // }
                }
                State::ExpectAttrExpression => {}
            }

            holder.replace(state);
        }

        fn state(state_history: &mut Vec<Option<State>>) -> (&mut Option<State>, State) {
            let holder = state_history
                .last_mut()
                .expect("Should have at least one state set (State::Start).");
            let taken = holder.take().expect("Should be set.");
            (holder, taken)
        }

        blocks
    }
}

#[derive(PartialEq)]
enum State {
    Start,
    BuildingBlock(BlockBuilder),
    BuildingAttr(AttrBuilder),
    ExpectAttrExpression,
}
