use super::TokenType::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
    start: usize,
    end: usize,
    r#type: TokenType,
}

impl Token {
    pub fn new(start: usize, r#type: TokenType) -> Token {
        // match and validate start/end with token len

        if r#type.len() == 0 {
            panic!("Token should not be 0 length. TokenType: {}", r#type)
        }

        Token {
            start,
            end: start + r#type.len(),
            r#type,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn r#type(&self) -> &TokenType {
        &self.r#type
    }

    #[cfg(test)]
    pub fn test(&self, source: &str) -> bool {
        match &self.r#type {
            BlockIdentifier(str) |
            LineComment(str) |
            BlockComment(str) |
            AttrIdentifier(str) |
            DirectiveIdentifier(str) |
            EventListenerIdentifier(str) |
            EventListenerHandler(str) |
            DirectiveValue(str) |
            ExprString(str) |
            ExprNumber(str) |
            ExprVariable(str) |
            ExprFunctionCall(str) => &source[self.start..self.end] == str,

            Colon => &source[self.start..self.end] == ":",
            BlockOpen => &source[self.start..self.end] == "{",
            BlockClose => &source[self.start..self.end] == "}",
            AttrColon => &source[self.start..self.end] == ":",
            AttrSeparator => &source[self.start..self.end] == ",",
            DirectiveOpen => &source[self.start..self.end] == "[",
            DirectiveClose => &source[self.start..self.end] == "]",
            DirectiveColon => &source[self.start..self.end] == ":",
            EventListenerOpen => &source[self.start..self.end] == "@[",
            EventListenerClose => &source[self.start..self.end] == "]",
            EventListenerColon => &source[self.start..self.end] == ":",
            ExprParenthesesOpen => &source[self.start..self.end] == "(",
            ExprParenthesesClose => &source[self.start..self.end] == ")",

            ExprOperator(_operator) => { todo!("Test operator.") }
            Expression => todo!("Test expression."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keyword(KeywordEnum),
    BlockIdentifier(String),
    LineComment(String),
    BlockComment(String),
    Colon, // todo remove
    BlockOpen,
    BlockClose,
    AttrIdentifier(String),
    AttrColon,
    AttrSeparator,
    Expression,
    DirectiveOpen,
    DirectiveClose,
    DirectiveIdentifier(String),
    DirectiveColon,
    DirectiveValue(String),
    EventListenerOpen,
    EventListenerClose,
    EventListenerIdentifier(String),
    EventListenerColon,
    EventListenerHandler(String),

    ExprString(String),
    ExprNumber(String),
    ExprVariable(String),
    ExprFunctionCall(String),
    ExprOperator(Operator),
    ExprParenthesesOpen,
    ExprParenthesesClose,
}

impl TokenType {
    pub fn len(&self) -> usize {
        match self {
            BlockIdentifier(str) |
            LineComment(str) |
            BlockComment(str) |
            AttrIdentifier(str) |
            DirectiveIdentifier(str) |
            DirectiveValue(str) |
            EventListenerHandler(str) |
            ExprString(str) |
            ExprNumber(str) |
            ExprVariable(str) |
            ExprFunctionCall(str) |
            EventListenerIdentifier(str) => str.len(),

            Colon |
            BlockOpen |
            BlockClose |
            AttrColon |
            AttrSeparator |
            Expression |
            DirectiveOpen |
            DirectiveClose |
            DirectiveColon |
            EventListenerClose |
            EventListenerColon |
            ExprParenthesesOpen |
            ExprParenthesesClose => 1,

            EventListenerOpen => 2,

            ExprOperator(op) => {
                op.symbol().len()
            }
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlockIdentifier(str) => write!(f, "BlockIdentifier({str})"),
            LineComment(str) => write!(f, "LineComment({str})"),
            BlockComment(str) => write!(f, "BlockComment({str})"),
            Colon => write!(f, "Colon"),
            BlockOpen => write!(f, "BlockOpen"),
            BlockClose => write!(f, "BlockClose"),
            AttrIdentifier(str) => write!(f, "AttrIdentifier({str})"),
            AttrColon => write!(f, "AttrColon"),
            AttrSeparator => write!(f, "AttrSeparator"),
            Expression => write!(f, "Expression"),
            DirectiveOpen => write!(f, "DirectiveOpen"),
            DirectiveClose => write!(f, "DirectiveClose"),
            DirectiveIdentifier(_) => write!(f, "DirectiveIdentifier"),
            DirectiveColon => write!(f, "DirectiveColon"),
            DirectiveValue(str) => write!(f, "DirectiveValue({str})"),
            EventListenerOpen => write!(f, "EventListenerOpen"),
            EventListenerClose => write!(f, "EventListenerClose"),
            EventListenerIdentifier(str) => write!(f, "EventListenerIdentifier({str})"),
            EventListenerColon => write!(f, "EventListenerColon"),
            EventListenerHandler(str) => write!(f, "EventListenerHandler({str})"),

            // Expression token
            ExprString(str) => write!(f, "ExprString({str})"),
            ExprNumber(str) => write!(f, "ExprNumber({str})"),
            ExprVariable(str) => write!(f, "ExprVariable({str})"),
            ExprFunctionCall(str) => write!(f, "ExprFunctionCall({str})"),
            ExprOperator(op) => write!(f, "ExprOperator({op})"),
            ExprParenthesesOpen => write!(f, "ExprParenthesesOpen"),
            ExprParenthesesClose => write!(f, "ExprParenthesesClose"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic
    Add,
    Sub,
    Multiply,
    Power, // ^x
    Divide,
    Modulus,

    // Assignment operators
    Assign,      // =
    Increment,   // ++
    Decrement,   // --
    IncrementBy, // +=
    DecrementBy, // -=
    MultiplyBy,  // *=
    DivideBy,    // /=

    // Comparisons
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,

    // Logical
    And,
    Or,
    Not,
}

impl Operator {
    pub(super) fn is_operator_char(char: char) -> bool {
        ['+', '-', '*', '^', '/', '%', '=', '<', '>', '!', '&', '|'].contains(&char)
    }

    pub(super) fn get_match(buf: &str) -> OperatorMatchResult {
        let variants = [
            Operator::Add,
            Operator::Sub,
            Operator::Multiply,
            Operator::Power,
            Operator::Divide,
            Operator::Modulus,
            Operator::Assign,
            Operator::Increment,
            Operator::Decrement,
            Operator::IncrementBy,
            Operator::DecrementBy,
            Operator::MultiplyBy,
            Operator::DivideBy,
            Operator::EqualTo,
            Operator::NotEqualTo,
            Operator::GreaterThan,
            Operator::GreaterThanOrEqualTo,
            Operator::LessThan,
            Operator::LessThanOrEqualTo,
            Operator::And,
            Operator::Or,
            Operator::Not,
        ];

        for variant in variants.iter() {
            if variant.symbol() == buf {
                return OperatorMatchResult::Matched(variant.clone());
            }
        }

        OperatorMatchResult::Failed
    }

    pub(super) fn symbol(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Multiply => "*",
            Operator::Power => "^",
            Operator::Divide => "/",
            Operator::Modulus => "%",
            Operator::Assign => "=",
            Operator::Increment => "++",
            Operator::Decrement => "--",
            Operator::IncrementBy => "+=",
            Operator::DecrementBy => "-=",
            Operator::MultiplyBy => "*=",
            Operator::DivideBy => "/=",
            Operator::EqualTo => "==",
            Operator::NotEqualTo => "!=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanOrEqualTo => ">=",
            Operator::LessThan => "<",
            Operator::LessThanOrEqualTo => "<=",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Not => "!",
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "Add"),
            Operator::Sub => write!(f, "Sub"),
            Operator::Multiply => write!(f, "Multiply"),
            Operator::Power => write!(f, "Power"),
            Operator::Divide => write!(f, "Divide"),
            Operator::Modulus => write!(f, "Modulus"),
            Operator::Assign => write!(f, "Assign"),
            Operator::Increment => write!(f, "Increment"),
            Operator::Decrement => write!(f, "Decrement"),
            Operator::IncrementBy => write!(f, "IncrementBy"),
            Operator::DecrementBy => write!(f, "DecrementBy"),
            Operator::MultiplyBy => write!(f, "MultiplyBy"),
            Operator::DivideBy => write!(f, "DivideBy"),
            Operator::EqualTo => write!(f, "EqualTo"),
            Operator::NotEqualTo => write!(f, "NotEqualTo"),
            Operator::GreaterThan => write!(f, "GreaterThan"),
            Operator::GreaterThanOrEqualTo => write!(f, "GreaterThanOrEqualTo"),
            Operator::LessThan => write!(f, "LessThan"),
            Operator::LessThanOrEqualTo => write!(f, "LessThanOrEqualTo"),
            Operator::And => write!(f, "And"),
            Operator::Or => write!(f, "Or"),
            Operator::Not => write!(f, "Not"),
        }
    }
}

pub(super) enum OperatorMatchResult {
    Matched(Operator),
    Failed,
}
