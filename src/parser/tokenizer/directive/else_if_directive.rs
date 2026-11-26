use crate::parser::tokenizer::directive::{Directive, DirectiveValueType, add_expr_token};
use crate::parser::tokenizer::token::ExprToken;
use crate::parser::tree_builder::postfix_processor::PostfixProcessor;

#[derive(Debug)]
pub struct ElseIfDirective<'token> {
    postfix_processor: PostfixProcessor<'token>,
}

impl<'token> ElseIfDirective<'token> {
    pub fn new() -> Self {
        Self {
            postfix_processor: PostfixProcessor::new(),
        }
    }

    fn postfix_processor_mut(&mut self) -> &mut PostfixProcessor<'token> {
        &mut self.postfix_processor
    }
}

impl<'token> Directive<'token> for ElseIfDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }

    fn add_expr_token(&mut self, token: &'token ExprToken) {
        add_expr_token(&mut self.postfix_processor, token)
    }
}
