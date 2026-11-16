use std::fmt::Debug;

pub trait Directive: Debug {
    fn parse(&mut self, char: char) -> DirectiveParseResult;
    fn value_type(&self) -> DirectiveValueType;
}

#[derive(Debug)]
pub struct DirectiveIf {}

impl Directive for DirectiveIf {
    fn parse(&mut self, char: char) -> DirectiveParseResult {
        DirectiveParseResult::Void
    }

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
    fn parse(&mut self, char: char) -> DirectiveParseResult {
        // Since this directive has no value, just return done.
        DirectiveParseResult::Done
    }

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
    fn parse(&mut self, char: char) -> DirectiveParseResult {
        DirectiveParseResult::Void
    }

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
    fn parse(&mut self, char: char) -> DirectiveParseResult {
        todo!()
    }

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
    fn parse(&mut self, char: char) -> DirectiveParseResult {
        DirectiveParseResult::Void
    }

    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::String
    }
}

impl PartialEq for DirectiveExtends {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// This is for testing compiler
#[derive(Debug)]
pub struct DirectiveTest {}

impl Directive for DirectiveTest {
    fn parse(&mut self, char: char) -> DirectiveParseResult {
        DirectiveParseResult::Void
    }

    fn value_type(&self) -> DirectiveValueType {
        DirectiveValueType::None
    }
}

impl PartialEq for DirectiveTest {
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
