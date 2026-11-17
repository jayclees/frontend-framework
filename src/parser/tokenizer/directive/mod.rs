use crate::parser::tokenizer::token::ExprToken;
use std::fmt::Debug;

pub trait Directive: Debug {
    fn value_type(&self) -> DirectiveValueType;
}

#[derive(Debug)]
pub struct DirectiveIf<'a> {
    tokens: Vec<&'a ExprToken>,
}

impl<'a> DirectiveIf<'a> {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }
}

impl<'a> Directive for DirectiveIf<'a> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }
}

impl<'a> PartialEq for DirectiveIf<'a> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct DirectiveElse {}

impl DirectiveElse {
    pub fn new() -> Self {
        Self {}
    }
}

impl Directive for DirectiveElse {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }
}

#[derive(Debug)]
pub struct DirectiveElseIf<'a> {
    tokens: Vec<&'a ExprToken>,
}

impl<'a> DirectiveElseIf<'a> {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }
}

impl<'a> Directive for DirectiveElseIf<'a> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }
}

#[derive(Debug)]
pub struct DirectiveFor {}

impl DirectiveFor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Directive for DirectiveFor {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Custom
    }
}

#[derive(Debug)]
pub struct DirectiveExtends<'a> {
    layout: Option<&'a String>,
}

impl<'a> DirectiveExtends<'a> {
    pub fn new() -> Self {
        Self { layout: None }
    }
}

impl<'a> Directive for DirectiveExtends<'a> {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::String
    }
}

/// This is for testing
#[derive(Debug)]
pub struct DirectiveDummyMarker {}

impl<'a> DirectiveDummyMarker {
    pub fn new() -> Self {
        Self {}
    }
}

impl Directive for DirectiveDummyMarker {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }
}

/// This is for testing
#[derive(Debug)]
pub struct DirectiveDummyString<'a> {
    layout: Option<&'a String>,
}

impl<'a> DirectiveDummyString<'a> {
    pub fn new() -> Self {
        Self { layout: None }
    }
}

impl<'a> Directive for DirectiveDummyString<'a> {
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
    Custom,
    None,
}
