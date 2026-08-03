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

    ExprString,
    ExprNumber,
    ExprVariable,
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
            ExprString => write!(f, "ExprString"),
            ExprNumber => write!(f, "ExprNumber"),
            ExprVariable => write!(f, "ExprVariable"),
            ExprFunctionCall => write!(f, "ExprFunctionCall"),
            ExprOperator(Symbol) => write!(f, ")"),
            ExprParenthesesOpen => write!(f, "ExprParenthesesOpen"),
            ExprParenthesesClose => write!(f, "ExprParenthesesClose"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Variable(String),

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
