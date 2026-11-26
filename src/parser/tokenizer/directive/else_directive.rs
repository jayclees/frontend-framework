use crate::parser::tokenizer::directive::{Directive, DirectiveValueType};
use crate::parser::tokenizer::token::ExprToken;

#[derive(Debug)]
pub struct ElseDirective {}

impl ElseDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'token> Directive<'token> for ElseDirective {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }

    fn add_expr_token(&mut self, token: &'token ExprToken) {
        panic!("Cannot add token to `else` directive since it takes no value.")
    }
}
