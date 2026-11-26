use crate::parser::tokenizer::directive::{Directive, DirectiveValueType};
use crate::parser::tokenizer::token::ExprToken;

/// This is for testing
#[derive(Debug)]
pub struct DummyMarkerDirective {}

impl DummyMarkerDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'token> Directive<'token> for DummyMarkerDirective {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }

    fn add_expr_token(&mut self, _: &'token ExprToken) {
        panic!("Cannot add token to marker directive.")
    }
}
