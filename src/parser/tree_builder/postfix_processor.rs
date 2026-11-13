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
                Operator::Modulus => unimplemented!("PostfixProcessor@process(Modulus) not implemented."),
                Operator::Assign => unimplemented!("PostfixProcessor@process(Assign) not implemented."),
                Operator::Increment => unimplemented!("PostfixProcessor@process(Increment) not implemented."),
                Operator::Decrement => unimplemented!("PostfixProcessor@process(Decrement) not implemented."),
                Operator::IncrementBy => unimplemented!("PostfixProcessor@process(IncrementBy) not implemented."),
                Operator::DecrementBy => unimplemented!("PostfixProcessor@process(DecrementBy) not implemented."),
                Operator::MultiplyBy => unimplemented!("PostfixProcessor@process(MultiplyBy) not implemented."),
                Operator::DivideBy => unimplemented!("PostfixProcessor@process(DivideBy) not implemented."),
                Operator::EqualTo => unimplemented!("PostfixProcessor@process(EqualTo) not implemented."),
                Operator::NotEqualTo => unimplemented!("PostfixProcessor@process(NotEqualTo) not implemented."),
                Operator::GreaterThan => unimplemented!("PostfixProcessor@process(GreaterThan) not implemented."),
                Operator::GreaterThanOrEqualTo => unimplemented!("PostfixProcessor@process(GreaterThanOrEqualTo) not implemented."),
                Operator::LessThan => unimplemented!("PostfixProcessor@process(LessThan) not implemented."),
                Operator::LessThanOrEqualTo => unimplemented!("PostfixProcessor@process(LessThanOrEqualTo) not implemented."),
                Operator::And => unimplemented!("PostfixProcessor@process(And) not implemented."),
                Operator::Or => unimplemented!("PostfixProcessor@process(Or) not implemented."),
                Operator::Not => unimplemented!("PostfixProcessor@process(Not) not implemented."),
                Operator::Concatenation => unimplemented!("PostfixProcessor@process(Concatenation) not implemented."),
            },
            ExprToken::String(_) => {}
            ExprToken::Var(_) => {}
            ExprToken::Dot => {}
            ExprToken::BracketAccessorOpen => {}
            ExprToken::BracketAccessorClose => {}
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
                            Operator::Concatenation => buf.push(ExprToken::String(format!("{left}{right}"))),
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

        if let Some(ExprToken::Number(num)) = buf.first() {
            BlockContent::Text(num.to_owned())
        } else {
            panic!("Buf should not be empty.");
        }
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
