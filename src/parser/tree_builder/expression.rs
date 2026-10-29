use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::block::BlockContent;

#[derive(Debug)]
pub struct BinaryOperation {
    left: Expression,
    right: Expression,
    operator: Operator,
}

impl BinaryOperation {
    pub fn new(left: Expression, right: Expression, operator: Operator) -> BinaryOperation {
        BinaryOperation {
            left,
            right,
            operator,
        }
    }

    pub fn resolve(&self) -> BlockContent {
        let t = match self.operator {
            Operator::Plus => {},
            Operator::Minus => {}
            Operator::Multiply => {}
            Operator::Power => {}
            Operator::Divide => {}
            Operator::Modulus => {}
            Operator::Assign => {}
            Operator::Increment => {}
            Operator::Decrement => {}
            Operator::IncrementBy => {}
            Operator::DecrementBy => {}
            Operator::MultiplyBy => {}
            Operator::DivideBy => {}
            Operator::EqualTo => {}
            Operator::NotEqualTo => {}
            Operator::GreaterThan => {}
            Operator::GreaterThanOrEqualTo => {}
            Operator::LessThan => {}
            Operator::LessThanOrEqualTo => {}
            Operator::And => {}
            Operator::Or => {}
            Operator::Not => {}
        };
        // "foo"."bar".(1 + 1)
        // 1 + 2 * 3 / 4 ^ 2
        // parenthesis, exponent, multiplication, division, addition, subtraction
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

    // pub fn is_empty(&self) -> bool {
    //     self.parts.is_empty()
    // }

    // pub fn clear_tokens(&mut self) {
    //     self.parts.clear()
    // }
}

#[derive(Debug)]
pub enum Expression {
    Integer(usize),
    Float(f64),
    String(String),
    Variable {
        handle: String,
        value: Box<Expression>,
    },
    Function {
        handle: String,
        value: Box<Expression>,
        parameters: Vec<Expression>,
    },
}

pub enum ExpressionResult {
    Number(String),
    String(String),
}
