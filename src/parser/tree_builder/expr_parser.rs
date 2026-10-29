use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::expression::{BinaryOperation, Expression};

// test(arg1, arg2, 3, "hello", null)
pub struct ExprParser<'a> {
    buf: Vec<&'a ExprToken>,
}

impl<'a> ExprParser<'a> {
    pub fn new() -> ExprParser<'a> {
        ExprParser { buf: vec![] }
    }

    pub fn add(&mut self, tok: &'a ExprToken) {
        self.buf.push(tok);
    }

    pub fn build_tree(&self) -> BinaryOperation {
        BinaryOperation::new(
            Expression::String("left".to_owned()),
            Expression::String("right".to_owned()),
            Operator::Plus,
        )
    }
}

#[derive(Debug)]
pub enum ExpressionPart {
    OperandSingle(Vec<ExprToken>),
    OperandGroup(BinaryOperation),
    Operator(Operator),
}
