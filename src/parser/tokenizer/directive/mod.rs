use crate::parser::tokenizer::token::ExprToken;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) enum DirectiveEnum<'token> {
    If(IfDirective<'token>),
    ElseIf(ElseIfDirective<'token>),
    Else(ElseDirective),
    For(ForDirective),
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
            _ => todo!("Allow custom directives?")
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

pub trait Directive: Debug {
    fn value_type(&self) -> DirectiveValueType;
}

#[derive(Debug)]
pub struct IfDirective<'token> {
    tokens: Vec<&'token ExprToken>,
}

impl<'token> IfDirective<'token> {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }
}

impl<'token> Directive for IfDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }
}

impl<'token> PartialEq for IfDirective<'token> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct ElseDirective {}

impl ElseDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl Directive for ElseDirective {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }
}

#[derive(Debug)]
pub struct ElseIfDirective<'token> {
    tokens: Vec<&'token ExprToken>,
}

impl<'token> ElseIfDirective<'token> {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }
}

impl<'token> Directive for ElseIfDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }
}

#[derive(Debug)]
pub struct ForDirective {}

impl ForDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl Directive for ForDirective {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Custom
    }
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

impl<'token> Directive for ExtendsDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::String
    }
}

/// This is for testing
#[derive(Debug)]
pub struct DummyMarkerDirective {}

impl<'token> DummyMarkerDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl Directive for DummyMarkerDirective {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
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

impl<'token> Directive for DummyStringDirective<'token> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
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
