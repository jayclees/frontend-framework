use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::block::BlockContent;

/// Read:
/// - https://runestone.academy/ns/books/published/pythonds/BasicDS/InfixPrefixandPostfixExpressions.html
/// - https://www.geeksforgeeks.org/dsa/convert-infix-expression-to-postfix-expression/
#[derive(Debug)]
pub struct PostfixProcessor<'token> {
    op_stack: Vec<&'token ExprToken>,
    postfix_expr: Vec<Part<'token>>,
    step: usize,
    part_builders: Vec<Part<'token>>
}

impl<'token> PostfixProcessor<'token> {
    pub fn new() -> PostfixProcessor<'token> {
        PostfixProcessor {
            op_stack: vec![],
            postfix_expr: vec![],
            step: 0,
            part_builders: vec![],
        }
    }

    pub fn process(&mut self, token: &'token ExprToken) {
        match token {
            ExprToken::Number(_) => self.postfix_expr.push(Part::Operand(token)),
            ExprToken::Operator(op) => match op {
                Operator::Plus
                | Operator::Minus
                | Operator::Multiply
                | Operator::Divide
                | Operator::Power => {
                    // Pop each operator on the stack that has higher
                    // or equal precedence and append to output.
                    for i in (0..self.op_stack.len()).rev() {
                        let to_pop_token = self.op_stack.get(i);
                        if let Some(ExprToken::Operator(to_pop_op)) = to_pop_token
                            && to_pop_op.precedence() >= op.precedence()
                        {
                            let popped = self.op_stack.remove(i);
                            self.postfix_expr.push(Part::Operator(popped));
                        }
                    }
                    self.op_stack.push(token);
                }
                Operator::ParenthesesOpen => {
                    self.op_stack.push(token);
                }
                Operator::ParenthesesClose => {
                    // Pop all operators and append to output until a left parenthesis is found.
                    for i in (0..self.op_stack.len()).rev() {
                        let to_pop_token = self.op_stack.get(i);
                        if let Some(ExprToken::Operator(Operator::ParenthesesOpen)) = to_pop_token {
                            self.op_stack.remove(i);
                            break;
                        } else {
                            let to_add_op = self.op_stack.remove(i);
                            self.postfix_expr.push(Part::Operator(to_add_op))
                        }
                    }
                }
                Operator::Modulus
                | Operator::Assign
                | Operator::Increment
                | Operator::Decrement
                | Operator::IncrementBy
                | Operator::DecrementBy
                | Operator::MultiplyBy
                | Operator::DivideBy
                | Operator::EqualTo
                | Operator::NotEqualTo
                | Operator::GreaterThan
                | Operator::GreaterThanOrEqualTo
                | Operator::LessThan
                | Operator::LessThanOrEqualTo
                | Operator::And
                | Operator::Or
                | Operator::Not => unimplemented!("Operator not implemented."),
            },
            ExprToken::String(_) => {
                self.postfix_expr.push(Part::Operand(token));
            }
            ExprToken::Var(_) => unimplemented!(),
            ExprToken::PropAccessorVar(_) => unimplemented!(),
            ExprToken::PropAccessorDot => unimplemented!(),
            ExprToken::UnresolvedDot => unimplemented!(),
            ExprToken::BracketAccessorOpen => unimplemented!(),
            ExprToken::BracketAccessorClose => unimplemented!(),
            ExprToken::FunctionArgsOpen => unimplemented!(),
            ExprToken::FunctionArgsClose => unimplemented!(),
            ExprToken::ArrayOpen => unimplemented!(),
            ExprToken::ArrayClose => unimplemented!(),
            ExprToken::True => unimplemented!(),
            ExprToken::False => unimplemented!(),
            ExprToken::Null => unimplemented!(),
        }
        self.step += 1;
    }

    pub fn resolve(&mut self) -> BlockContent {
        // Add remaining operators to list
        for operator in self.op_stack.iter().rev() {
            self.postfix_expr.push(Part::Operator(operator));
        }

        // dbg!(&self.output);

        let mut output: Vec<Part> = std::mem::take(&mut self.postfix_expr);
        let mut buf: Vec<ExprToken> = vec![];

        while !output.is_empty() {
            match output.remove(0) {
                Part::Operand(operand) => {
                    buf.push(operand.clone());
                }
                Part::Operator(operator) => {
                    let right = buf.pop().expect("Operand buffer is empty.");
                    let left = buf.pop().expect("Only one operand left in buffer.");

                    if let ExprToken::Operator(operator) = operator {
                        match operator {
                            Operator::Plus => {
                                // todo should we separate Operator::Plus into Operator::Concat?
                                // todo potentially to have different operator precedence levels.
                                if matches!(left, ExprToken::String(_)) || matches!(right, ExprToken::String(_)) {
                                    buf.push(ExprToken::Number(left.concat(right)))
                                } else {
                                    buf.push(ExprToken::Number(left.add(right)))
                                }
                            },
                            Operator::Minus => buf.push(ExprToken::Number(left.subtract(right))),
                            Operator::Multiply => buf.push(ExprToken::Number(left.multiply(right))),
                            Operator::Divide => buf.push(ExprToken::Number(left.divide(right))),
                            Operator::Power => buf.push(ExprToken::Number(left.power(right))),
                            Operator::Modulus => buf.push(ExprToken::Number(left.modulus(right))),
                            // todo move this into Operator::Plus match arm and conditionally handle
                            // todo based on operand types
                            // Operator::Concatenation => {
                            //     buf.push(ExprToken::String(format!("{left}{right}")))
                            // }
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
                            Operator::ParenthesesOpen => {}
                            Operator::ParenthesesClose => {}
                            Operator::And => {}
                            Operator::Or => {}
                            Operator::Not => {}
                        }
                    } else {
                        // todo Should this be checked earlier by different enum strategy?
                        panic!("Non operator found in Part::Operator.")
                    }
                }
            }
        }

        // dbg!(&buf);

        self.reset();

        // dbg!(buf.first());
        if let Some(ExprToken::Number(num)) = buf.first() {
            BlockContent::Text(num.to_owned())
        } else if let Some(ExprToken::String(str)) = buf.first() {
            let chars = str.chars();
            let string = chars.skip(1).take(str.len() - 2).collect::<String>();
            BlockContent::Text(string)
        } else {
            panic!("Buf should not be empty.");
        }
    }

    pub fn is_empty(&self) -> bool {
        self.postfix_expr.is_empty()
    }

    pub fn reset(&mut self) {
        self.postfix_expr = vec![];
        self.op_stack = vec![];
    }
}

#[derive(Debug, Clone)]
enum Part<'a> {
    Operand(&'a ExprToken),
    Operator(&'a ExprToken),
}
// enum Part<'token> {
//     Number(&'token ExprToken),
//     String(&'token ExprToken),
//     Var(&'token ExprToken),
//     FnCall(&'token ExprToken),
//     Operator(&'token ExprToken),
// }

impl<'token> Part<'token> {
    fn get_operand(&self) -> &'token ExprToken {
        match self {
            Part::Operand(token) => token,
            Part::Operator(token) => panic!(r#""{token}" is not an operand."#),
        }
    }

    fn get_operator(&self) -> &'token ExprToken {
        match self {
            Part::Operator(token) => token,
            Part::Operand(token) => panic!(r#""{token}" is not an operator."#),
        }
    }
}
