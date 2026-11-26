use crate::parser::tokenizer::token::ExprToken;
use crate::parser::tree_builder::postfix_processor::{PostfixProcessor, ValueType};
use serde_json::{Number, Value};
use std::assert_matches;
use std::fmt::Debug;
use crate::parser::tree_builder::value_as_bool;

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
