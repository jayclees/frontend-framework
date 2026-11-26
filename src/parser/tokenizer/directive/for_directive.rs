use crate::parser::tokenizer::directive::{Directive, DirectiveValueType};
use crate::parser::tokenizer::token::ExprToken;
use crate::parser::tree_builder::postfix_processor::{PostfixProcessor, ValueType};
use serde_json::Value;
use std::assert_matches;

#[derive(Debug)]
pub struct ForDirective<'token> {
    state: ForDirectiveBuildState,
    item_var: Option<&'token ExprToken>,
    index_var: Option<&'token ExprToken>,
    expr_processor: PostfixProcessor<'token>,
}

impl<'token> ForDirective<'token> {
    pub fn new() -> Self {
        Self {
            state: ForDirectiveBuildState::ReceiveItemVar,
            item_var: None,
            index_var: None,
            expr_processor: PostfixProcessor::new(),
        }
    }

    pub fn set_state(&mut self, state: ForDirectiveBuildState) {
        self.state = state;
    }

    pub fn evaluate<'state>(&mut self, state: &'state Value) -> ValueType<'state> {
        self.expr_processor.convert().evaluate(vec![state])
    }

    pub fn item_var(&self) -> &'token String {
        match self.item_var {
            Some(ExprToken::Var(str)) => str,
            _ => panic!("Should not be reached.")
        }
    }

    pub fn index_var(&self) -> Option<&'token String> {
        match self.index_var {
            Some(ExprToken::Var(str)) => Some(str),
            None => None,
            token => {
                dbg!(token);
                panic!("Should not be reached.")
            }
        }
    }
}

impl<'token> Directive<'token> for ForDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::ForStatement
    }

    fn add_expr_token(&mut self, token: &'token ExprToken) {
        match self.state {
            ForDirectiveBuildState::ReceiveIndexVar => {
                assert_matches!(token, ExprToken::Var(_));
                self.index_var = Some(token);
            }
            ForDirectiveBuildState::ReceiveItemVar => {
                assert_matches!(token, ExprToken::Var(_));
                self.item_var = Some(token);
            }
            ForDirectiveBuildState::ReceiveIterExpr => self.expr_processor.add(token),
        }
    }
}

#[derive(Debug)]
pub enum ForDirectiveBuildState {
    ReceiveIndexVar,
    ReceiveItemVar,
    ReceiveIterExpr,
}