use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use super::TokenType::*;

#[derive(Debug)]
pub struct Token {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) r#type: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keyword(KeywordEnum),
    BlockIdentifier(String),
    StringExpr(String),
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
    ExprNumber,
    ExprVariable(String),
    ExprFunctionCall,
    ExprOperator(Symbol),
    ExprParenthesesOpen,
    ExprParenthesesClose,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlockIdentifier(str) => write!(f, "BlockIdentifier({str})"),
            StringExpr(str) => write!(f, "StringExpr({str})"),
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
            ExprNumber => write!(f, "ExprNumber"),
            ExprVariable(str) => write!(f, "ExprVariable({str})"),
            ExprFunctionCall => write!(f, "ExprFunctionCall"),
            ExprOperator(symbol) => write!(f, "ExprOperator({symbol}"),
            ExprParenthesesOpen => write!(f, "ExprParenthesesOpen"),
            ExprParenthesesClose => write!(f, "ExprParenthesesClose"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
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

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Add => write!(f, "Add"),
            Symbol::Sub => write!(f, "Sub"),
            Symbol::Multiply => write!(f, "Multiply"),
            Symbol::Power => write!(f, "Power"),
            Symbol::Divide => write!(f, "Divide"),
            Symbol::Modulus => write!(f, "Modulus"),
            Symbol::Assign => write!(f, "Assign"),
            Symbol::Increment => write!(f, "Increment"),
            Symbol::Decrement => write!(f, "Decrement"),
            Symbol::IncrementBy => write!(f, "IncrementBy"),
            Symbol::DecrementBy => write!(f, "DecrementBy"),
            Symbol::MultiplyBy => write!(f, "MultiplyBy"),
            Symbol::DivideBy => write!(f, "DivideBy"),
            Symbol::EqualTo => write!(f, "EqualTo"),
            Symbol::NotEqualTo => write!(f, "NotEqualTo"),
            Symbol::GreaterThan => write!(f, "GreaterThan"),
            Symbol::GreaterThanOrEqualTo => write!(f, "GreaterThanOrEqualTo"),
            Symbol::LessThan => write!(f, "LessThan"),
            Symbol::LessThanOrEqualTo => write!(f, "LessThanOrEqualTo"),
            Symbol::And => write!(f, "And"),
            Symbol::Or => write!(f, "Or"),
            Symbol::Not => write!(f, "Not"),
        }
    }
}
