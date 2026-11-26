use crate::parser::tokenizer::directive::{Directive, DirectiveValueType};
use crate::parser::tokenizer::token::ExprToken;

/// This is for testing
#[derive(Debug)]
pub struct DummyStringDirective<'token> {
    layout: Option<&'token String>,
}

impl<'token> DummyStringDirective<'token> {
    pub fn new() -> Self {
        Self { layout: None }
    }
}

impl<'token> Directive<'token> for DummyStringDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }

    fn add_expr_token(&mut self, token: &'token ExprToken) {
        todo!()
    }
}
