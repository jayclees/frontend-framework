pub mod dummy_marker_directive;
pub mod dummy_string_directive;
pub mod else_directive;
pub mod else_if_directive;
pub mod extends_directive;
pub mod for_directive;
pub mod if_directive;

use crate::parser::tokenizer::token::ExprToken;
use crate::parser::tree_builder::postfix_processor::PostfixProcessor;
use dummy_marker_directive::DummyMarkerDirective;
use dummy_string_directive::DummyStringDirective;
use else_directive::ElseDirective;
use else_if_directive::ElseIfDirective;
use extends_directive::ExtendsDirective;
use for_directive::ForDirective;
use if_directive::IfDirective;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) enum DirectiveEnum<'token> {
    If(IfDirective<'token>),
    ElseIf(ElseIfDirective<'token>),
    Else(ElseDirective),
    For(ForDirective<'token>),
    Extends(ExtendsDirective<'token>),
    DummyMarker(DummyMarkerDirective),
    DummyString(DummyStringDirective<'token>),
}

impl<'token> DirectiveEnum<'token> {
    pub fn from(id: &str) -> Self {
        match id {
            "if" => Self::If(IfDirective::new()),
            "elseif" => Self::ElseIf(ElseIfDirective::new()),
            "else" => Self::Else(ElseDirective::new()),
            "for" => Self::For(ForDirective::new()),
            "extends" => Self::Extends(ExtendsDirective::new()),
            "dummymarker" => Self::DummyMarker(DummyMarkerDirective::new()),
            "dummystring" => Self::DummyString(DummyStringDirective::new()),
            _ => todo!("Allow custom directives?"),
        }
    }

    pub fn value_type(&self) -> DirectiveValueType {
        match self {
            DirectiveEnum::If(_) => DirectiveValueType::Expression,
            DirectiveEnum::ElseIf(_) => DirectiveValueType::Expression,
            DirectiveEnum::Else(_) => DirectiveValueType::None,
            DirectiveEnum::For(_) => DirectiveValueType::ForStatement,
            DirectiveEnum::DummyMarker(_) => DirectiveValueType::None,
            DirectiveEnum::DummyString(_) => DirectiveValueType::String,
            DirectiveEnum::Extends(_) => DirectiveValueType::String,
        }
    }
}

pub trait Directive<'token>: Debug {
    fn value_type(&self) -> DirectiveValueType;
    fn add_expr_token(&mut self, token: &'token ExprToken);
}

pub(crate) enum DirectiveParseResult {
    Handled,
    Err(String),
    Done,
    Void,
}

pub(crate) enum DirectiveValueType {
    Expression,
    String,
    ForStatement,
    Custom,
    None,
}

fn add_expr_token<'token>(
    postfix_processor: &mut PostfixProcessor<'token>,
    token: &'token ExprToken,
) {
    postfix_processor.add(token);
}

fn set_string<'token>(string: &mut Option<&'token String>, token: &'token ExprToken) {
    match token {
        ExprToken::String(str) => {
            if string.is_some() {
                panic!("Layout already set for `extends` directive.")
            }
            string.replace(str);
        }
        _ => panic!(),
    }
}
