use std::fmt::Debug;

pub trait Directive: Debug {
    fn value_type(&self) -> DirectiveValueType;
}

#[derive(Debug)]
pub struct DirectiveIf {}

impl Directive for DirectiveIf {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }
}

impl PartialEq for DirectiveIf {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct DirectiveElse {}

impl Directive for DirectiveElse {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }
}

impl PartialEq for DirectiveElse {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct DirectiveElseIf {}

impl Directive for DirectiveElseIf {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Expression
    }
}

impl PartialEq for DirectiveElseIf {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct DirectiveFor {}

impl Directive for DirectiveFor {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::Custom
    }
}

impl PartialEq for DirectiveFor {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct DirectiveExtends {}

impl Directive for DirectiveExtends {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::String
    }
}

impl PartialEq for DirectiveExtends {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// This is for testing
#[derive(Debug)]
pub struct DirectiveDummyMarker {}

impl Directive for DirectiveDummyMarker {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }
}

impl PartialEq for DirectiveDummyMarker {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// This is for testing
#[derive(Debug)]
pub struct DirectiveDummyString {}

impl Directive for DirectiveDummyString {
    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }
}

impl PartialEq for DirectiveDummyString {
    fn eq(&self, other: &Self) -> bool {
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
    Custom,
    None,
}
