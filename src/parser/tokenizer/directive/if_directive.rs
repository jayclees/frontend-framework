use crate::parser::tokenizer::directive::{Directive, DirectiveValueType, add_expr_token};
use crate::parser::tokenizer::token::ExprToken;
use crate::parser::tree_builder::postfix_processor::{PostfixProcessor, ValueType};
use crate::parser::tree_builder::value_as_bool;
use serde_json::Value;

#[derive(Debug)]
pub struct IfDirective<'token> {
    postfix_processor: PostfixProcessor<'token>,
}

impl<'token> IfDirective<'token> {
    pub fn new() -> Self {
        Self {
            postfix_processor: PostfixProcessor::new(),
        }
    }

    fn add_expr_token(&mut self, token: &'token ExprToken) {
        add_expr_token(&mut self.postfix_processor, token)
    }

    pub fn evaluate_is_true(&mut self, state: &Value) -> bool {
        // todo extract into is_truthy helper function?
        match self.postfix_processor.convert().evaluate(vec![state]) {
            ValueType::Reference(val) => value_as_bool(val),
            ValueType::New(val) => value_as_bool(&val),
        }
    }
}

impl<'token> Directive<'token> for IfDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }

    fn add_expr_token(&mut self, token: &'token ExprToken) {
        add_expr_token(&mut self.postfix_processor, token)
    }
}

impl<'token> PartialEq for IfDirective<'token> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
