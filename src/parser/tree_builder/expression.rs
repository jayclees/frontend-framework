use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::block::BlockContent;

#[derive(Debug)]
pub struct BinaryOperation<'b, 'a: 'b> {
    buf: Vec<&'a ExprToken>,
    left: Option<Operand<'a, 'b>>,
    right: Option<Operand<'a, 'b>>,
    operator: Option<&'b Operator>,
}

impl<'a, 'b> BinaryOperation<'a, 'b> {
    pub fn empty() -> BinaryOperation<'a, 'b> {
        BinaryOperation {
            buf: vec![],
            left: None,
            right: None,
            operator: None,
        }
    }

    pub fn process_token(&mut self, token: &'a ExprToken) {
        let operand_or_operator = match token {
            ExprToken::String(str) => {
                OperandOrOperator::Operand(Operand::String(str.to_owned()))
            }
            ExprToken::Number(num) => {
                OperandOrOperator::Operand(Operand::Number(num.to_owned()))
            }
            ExprToken::Operator(op) => OperandOrOperator::Operator(op),
            ExprToken::Var(var) => todo!("Var"),
            ExprToken::Dot => todo!("DotAccessor"),
            ExprToken::BracketAccessorOpen => todo!("BracketAccessorOpen"),
            ExprToken::BracketAccessorClose => todo!("BracketAccessorClose"),
            ExprToken::FunctionArgsOpen => todo!("FunctionArgsOpen"),
            ExprToken::FunctionArgsClose => todo!("FunctionArgsClose"),
            ExprToken::ArrayOpen => todo!("ArrayOpen"),
            ExprToken::ArrayClose => todo!("ArrayClose"),
        };

        self.push_operand_or_operator(operand_or_operator);
    }

    fn push_operand_or_operator(&mut self, operand_or_operator: OperandOrOperator<'a, 'b>) {
        match operand_or_operator {
            OperandOrOperator::Operand(operand) => {
                if self.left.is_none() {
                    self.left = Some(operand)
                } else if self.right.is_none() {
                    self.right = Some(operand)
                } else if let Some(Operand::BinaryOperation(left)) = self.left.as_mut() {
                    left.push_operand_or_operator(OperandOrOperator::Operand(operand));
                } else if let Some(Operand::BinaryOperation(right)) = self.right.as_mut() {
                    right.push_operand_or_operator(OperandOrOperator::Operand(operand));
                } else {
                    unimplemented!();
                }
            }
            OperandOrOperator::Operator(operator) => {
                if self.operator.is_none() {
                    // todo determine if operator is valid
                    // for example if either side is a string, you cannot
                    // have any operator other than addition
                    self.operator = Some(operator);
                } else {
                    panic!("Operator already set.")
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    // pub fn resolve(&self) -> BlockContent {
    //     let left = match self.left.as_ref() {
    //         Some(op) => op,
    //         None => panic!("No left side operand set for binary operation."),
    //     };
    //     let right = match self.right.as_ref() {
    //         Some(op) => op,
    //         None => panic!("No right side operand set for binary operation."),
    //     };
    //     let operator = match self.operator.as_ref() {
    //         Some(op) => op,
    //         None => panic!("No operator set for binary operation."),
    //     };
    //     let concat = matches!(self.left, Some(Operand::String(_)))
    //         || matches!(self.right, Some(Operand::String(_)));
    //
    //     if let Operand::Number(l_num) = left && let Operand::Number(r_num) = right {
    //         let l_num = l_num.parse::<usize>().expect("Failed to convert to usize.");
    //         let r_num = r_num.parse::<usize>().expect("Failed to convert to usize.");
    //         let result = match operator {
    //             Operator::Plus => {
    //                 l_num + r_num
    //             }
    //             Operator::Minus => 0,
    //             Operator::Multiply => 0,
    //             Operator::Power => 0,
    //             Operator::Divide => 0,
    //             Operator::Modulus => 0,
    //             Operator::Assign => 0,
    //             Operator::Increment => 0,
    //             Operator::Decrement => 0,
    //             Operator::IncrementBy => 0,
    //             Operator::DecrementBy => 0,
    //             Operator::MultiplyBy => 0,
    //             Operator::DivideBy => 0,
    //             Operator::EqualTo => 0,
    //             Operator::NotEqualTo => 0,
    //             Operator::GreaterThan => 0,
    //             Operator::GreaterThanOrEqualTo => 0,
    //             Operator::LessThan => 0,
    //             Operator::LessThanOrEqualTo => 0,
    //             Operator::ParenthesesOpen => 0,
    //             Operator::ParenthesesClose => 0,
    //             Operator::And => 0,
    //             Operator::Or => 0,
    //             Operator::Not => 0,
    //             Operator::Concatenation => {}
    //         };
    //
    //         // todo rename BlockContent to just Content?
    //         return BlockContent::Text(result.to_string());
    //     }
    //
    //     // let result = match operator {
    //     //     Operator::Plus => {
    //     //
    //     //     }
    //     //     Operator::Minus => {}
    //     //     Operator::Multiply => {}
    //     //     Operator::Power => {}
    //     //     Operator::Divide => {}
    //     //     Operator::Modulus => {}
    //     //     Operator::Assign => {}
    //     //     Operator::Increment => {}
    //     //     Operator::Decrement => {}
    //     //     Operator::IncrementBy => {}
    //     //     Operator::DecrementBy => {}
    //     //     Operator::MultiplyBy => {}
    //     //     Operator::DivideBy => {}
    //     //     Operator::EqualTo => {}
    //     //     Operator::NotEqualTo => {}
    //     //     Operator::GreaterThan => {}
    //     //     Operator::GreaterThanOrEqualTo => {}
    //     //     Operator::LessThan => {}
    //     //     Operator::LessThanOrEqualTo => {}
    //     //     Operator::And => {}
    //     //     Operator::Or => {}
    //     //     Operator::Not => {}
    //     // };
    //     // "foo"."bar".(1 + 1)
    //     // 1 + 2 * 3 / 4 ^ 2
    //     // parenthesis, exponent, multiplication, division, addition, subtraction
    //     BlockContent::Text("".to_owned())
    // }

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
pub enum Operand<'a, 'b> {
    Number(String),
    String(String),
    Variable {
        handle: String,
        value: Box<Operand<'a, 'b>>,
    },
    Function {
        handle: String,
        value: Box<Operand<'a, 'b>>,
        parameters: Vec<Operand<'a, 'b>>,
    },
    BinaryOperation(Box<BinaryOperation<'a, 'b>>),
}

pub enum ExpressionResult {
    Number(String),
    String(String),
}

enum OperandOrOperator<'a, 'b> {
    Operand(Operand<'a, 'b>),
    Operator(&'b Operator),
}
