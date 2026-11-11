use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::block::BlockContent;

/// Read:
/// - https://runestone.academy/ns/books/published/pythonds/BasicDS/InfixPrefixandPostfixExpressions.html
/// - https://www.geeksforgeeks.org/dsa/convert-infix-expression-to-postfix-expression/
#[derive(Debug)]
pub struct PostfixProcessor<'a> {
    op_stack: Vec<&'a ExprToken>,
    output: Vec<Part<'a>>,
    step: usize,
}

impl<'a> PostfixProcessor<'a> {
    pub fn new() -> PostfixProcessor<'a> {
        PostfixProcessor {
            op_stack: vec![],
            output: vec![],
            step: 0,
        }
    }

    pub fn process(&mut self, token: &'a ExprToken) {
        match token {
            ExprToken::Number(_) => self.output.push(Part::Operand(token)),
            ExprToken::Operator(op) => match op {
                Operator::Plus | Operator::Minus | Operator::Multiply | Operator::Divide | Operator::Power => {
                    // Pop each operator on the stack that has higher
                    // or equal precedence and append to output.
                    for i in (0..self.op_stack.len()).rev() {
                        let to_pop_token = self.op_stack.get(i);
                        if let Some(ExprToken::Operator(to_pop_op)) = to_pop_token
                            && to_pop_op.precedence() >= op.precedence()
                        {
                            let popped = self.op_stack.remove(i);
                            self.output.push(Part::Operator(popped));
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
                            self.output.push(Part::Operator(to_add_op))
                        }
                    }
                }
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
            },
            ExprToken::String(_) => {}
            ExprToken::Var(_) => {}
            ExprToken::VarDotAccessor => {}
            ExprToken::VarBracketAccessorOpen => {}
            ExprToken::VarBracketAccessorClose => {}
            ExprToken::FunctionArgsOpen => {}
            ExprToken::FunctionArgsClose => {}
            ExprToken::ArrayOpen => {}
            ExprToken::ArrayClose => {}
        }
        self.step += 1;
    }

    pub fn resolve(&mut self) -> BlockContent {
        // Add remaining operators to list
        for operator in self.op_stack.iter().rev() {
            self.output.push(Part::Operator(operator));
        }

        // dbg!(&self.output);

        let mut output: Vec<Part> = std::mem::take(&mut self.output);
        let mut buf: Vec<ExprToken> = vec![];
        let mut step = 0;

        while !output.is_empty() {
            step += 1;

            // if step == 5 {
            //     crate::dd!(buf, output.remove(0));
            // }

            match output.remove(0) {
                Part::Operand(operand) => {
                    buf.push(operand.clone());
                }
                Part::Operator(operator) => {
                    let right = buf.pop().expect("Operand buffer is empty.");
                    let left = buf.pop().expect("Only one operand left in buffer.");

                    if let ExprToken::Operator(operator) = operator {
                        match operator {
                            Operator::Plus => buf.push(ExprToken::Number(left.add(right))),
                            Operator::Minus => buf.push(ExprToken::Number(left.subtract(right))),
                            Operator::Multiply => buf.push(ExprToken::Number(left.multiply(right))),
                            Operator::Divide => buf.push(ExprToken::Number(left.divide(right))),
                            Operator::Power => buf.push(ExprToken::Number(left.power(right))),
                            Operator::Modulus => buf.push(ExprToken::Number(left.modulus(right))),
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

        dbg!(&buf);

        self.reset();

        BlockContent::Text("hello".to_owned())
    }

    pub fn is_empty(&self) -> bool {
        self.output.is_empty()
    }

    pub fn reset(&mut self) {
        self.output = vec![];
        self.op_stack = vec![];
    }
}

#[derive(Debug, Clone)]
enum Part<'a> {
    Operand(&'a ExprToken),
    Operator(&'a ExprToken),
}

impl<'a> Part<'a> {
    fn get_operand(&self) -> &'a ExprToken {
        match self {
            Part::Operand(token) => token,
            Part::Operator(token) => panic!(r#""{token}" is not an operand."#),
        }
    }

    fn get_operator(&self) -> &'a ExprToken {
        match self {
            Part::Operator(token) => token,
            Part::Operand(token) => panic!(r#""{token}" is not an operator."#),
        }
    }
}
