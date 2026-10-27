use crate::parser::tokenizer::token::{Expr, Operator};
use crate::parser::tree_builder::block::BlockContent;

#[derive(Debug)]
pub struct Expression<'a> {
    parts: Vec<ExpressionPart<'a>>,
    sub_expressions: Vec<Expression<'a>>,
    buf: Vec<&'a Expr>,
}

impl<'a> Expression<'a> {
    pub fn new() -> Expression<'a> {
        Expression {
            parts: vec![],
            sub_expressions: vec![],
            buf: vec![],
        }
    }

    pub fn add_part(&mut self, token: &Expr) {
        if token == &Expr::ParenthesesOpen {
            for item in &self.parts {
                //
            }
        }
    }

    pub fn push_token(&mut self, token: &'a Expr) {
        self.parts.push(ExpressionPart::OperandSingle(vec![token]))
    }

    pub fn resolve_text(&self) -> BlockContent {
        BlockContent::Text("".to_owned())
    }

    // pub fn resolve_sub_expressions(&mut self) {
    //     // 1 + (2 / (3 + 1)) * (2 + 2) / ((2 - 1) ^ 2)
    //     let mut expressions: Vec<&Expr> = vec![];
    //     // pemdas
    //     // recursively find deepest level parentheses
    //     for tok in &self.parts {
    //         if *tok == &Expr::ParenthesesOpen {
    //             expressions.push(tok);
    //         }
    //     }
    // }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn clear_tokens(&mut self) {
        self.parts.clear()
    }
}

pub enum ExpressionResult {
    Number(String),
    String(String),
}

#[derive(Debug)]
pub enum ExpressionPart<'a> {
    OperandSingle(Vec<&'a Expr>),
    OperandGroup(Expression<'a>),
    Operator(Operator),
}
