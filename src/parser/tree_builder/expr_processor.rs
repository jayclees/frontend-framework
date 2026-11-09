use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::block::BlockContent;

#[derive(Debug)]
pub struct ExprProcessor<'a> {
    op_stack: Vec<&'a ExprToken>,
    postfix_output: Vec<Part<'a>>,
}

impl<'a> ExprProcessor<'a> {
    pub fn new() -> ExprProcessor<'a> {
        ExprProcessor {
            op_stack: vec![],
            postfix_output: vec![],
        }
    }

    pub fn process(&mut self, token: &'a ExprToken) {
        match token {
            ExprToken::Operator(operator) => {
                match operator {
                    Operator::Plus | Operator::Minus | Operator::Multiply | Operator::Divide => {
                        for (i, token) in self.op_stack.clone().iter().enumerate().rev() {
                            if let ExprToken::Operator(op) = token
                                && op.precedence() >= operator.precedence()
                            {
                                self.postfix_output
                                    .push(Part::Operator(self.op_stack.pop().expect("err")));
                                self.op_stack.push(token);
                            }
                        }
                    }
                    Operator::ParenthesesOpen => {
                        self.op_stack.push(token);
                    }
                    Operator::ParenthesesClose => {
                        for operator in self.op_stack.clone().iter().rev() {
                            match operator {
                                ExprToken::Operator(operator) => {
                                    if let Operator::ParenthesesOpen = operator {
                                        self.op_stack.pop();
                                        break;
                                    } else {
                                        let op_tok = self.op_stack.pop().expect("err");
                                        self.postfix_output.push(Part::Operator(op_tok));
                                    }
                                }

                                ExprToken::String(_) => unimplemented!(),
                                ExprToken::Number(_) => unimplemented!(),
                                ExprToken::Var(_) => unimplemented!(),
                                ExprToken::VarDotAccessor => unimplemented!(),
                                ExprToken::VarBracketAccessorOpen => unimplemented!(),
                                ExprToken::VarBracketAccessorClose => unimplemented!(),
                                ExprToken::FunctionArgsOpen => unimplemented!(),
                                ExprToken::FunctionArgsClose => unimplemented!(),
                                ExprToken::ArrayOpen => unimplemented!(),
                                ExprToken::ArrayClose => unimplemented!(),
                            }
                        }
                    }
                    t => {
                        crate::dd!(t);
                    }
                }
                self.op_stack.push(token);
            }

            ExprToken::String(_) => {}
            ExprToken::Number(_) => {
                self.postfix_output.push(Part::Operand(token));
            }
            ExprToken::Var(_) => {}
            ExprToken::VarDotAccessor => {}
            ExprToken::VarBracketAccessorOpen => {}
            ExprToken::VarBracketAccessorClose => {}
            ExprToken::FunctionArgsOpen => {}
            ExprToken::FunctionArgsClose => {}
            ExprToken::ArrayOpen => {}
            ExprToken::ArrayClose => {}
        }
    }

    pub fn build(&self) {
        //
    }

    pub fn resolve(&mut self) -> BlockContent {
        // add remaining operaters to list
        for operator in self.op_stack.iter().rev() {
            self.postfix_output.push(Part::Operator(operator));
        }

        dbg!(&self.postfix_output);
        BlockContent::Text("hello".to_owned())
    }

    pub fn is_empty(&self) -> bool {
        self.postfix_output.is_empty()
    }
}

#[derive(Debug)]
enum Part<'a> {
    Operand(&'a ExprToken),
    Operator(&'a ExprToken),
}
