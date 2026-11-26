use crate::parser::tokenizer::directive::{Directive, DirectiveValueType, set_string};
use crate::parser::tokenizer::token::ExprToken;

#[derive(Debug)]
pub struct ExtendsDirective<'token> {
    layout: Option<&'token String>,
}

impl<'token> ExtendsDirective<'token> {
    pub fn new() -> Self {
        Self { layout: None }
    }
}

impl<'token> Directive<'token> for ExtendsDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::String
    }

    fn add_expr_token(&mut self, token: &'token ExprToken) {
        set_string(&mut self.layout, token)
    }
}
