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
            panic!("Token should not be 0 length. TokenType: {}, cursor: {}", r#type, start)
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
        let source_str = &source[self.start..self.end];
        match &self.r#type {
            BlockIdentifier(str) |
            LineComment(str) |
            BlockComment(str) |
            AttrIdentifier(str) |
            DirectiveIdentifier(str) |
            EventListenerIdentifier(str) |
            EventListenerHandler(str) |
            DirectiveValue(str) => source_str == str,

            BlockOpen => source_str == "{",
            BlockClose => source_str == "}",
            AttrColon => source_str == ":",
            AttrSeparator => source_str == ",",
            DirectiveOpen => source_str == "[",
            DirectiveClose => source_str == "]",
            DirectiveColon => source_str == ":",
            EventListenerOpen => source_str == "@[",
            EventListenerClose => source_str == "]",
            EventListenerColon => source_str == ":",

            ExprToken(expr) => match expr {
                Expr::String(str) => source_str == str,
                Expr::Number(str) => source_str == str,
                Expr::Operator(op) => source_str == op.symbol(),
                Expr::Var(str) => source_str == str,
                Expr::VarDotAccessor => source_str == ".",
                Expr::VarBracketAccessorOpen => source_str == "[",
                Expr::VarBracketAccessorClose => source_str == "]",
                Expr::FunctionArgsOpen => source_str == "(",
                Expr::FunctionArgsClose => source_str == ")",
                Expr::ParenthesesOpen => source_str == "(",
                Expr::ParenthesesClose => source_str == ")",
                Expr::ArrayOpen => source_str == "[",
                Expr::ArrayClose => source_str == "]",
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keyword(KeywordEnum),
    BlockIdentifier(String),
    LineComment(String),
    BlockComment(String),
    BlockOpen,
    BlockClose,
    AttrIdentifier(String),
    AttrColon,
    AttrSeparator,
    ExprToken(Expr),
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
            EventListenerIdentifier(str) => str.len(),

            BlockOpen |
            BlockClose |
            AttrColon |
            AttrSeparator |
            DirectiveOpen |
            DirectiveClose |
            EventListenerClose |
            EventListenerColon |
            DirectiveColon => 1,

            EventListenerOpen => 2,

            ExprToken(expr) => expr.len(),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlockIdentifier(str) => write!(f, "BlockIdentifier({str})"),
            LineComment(str) => write!(f, "LineComment({str})"),
            BlockComment(str) => write!(f, "BlockComment({str})"),
            BlockOpen => write!(f, "BlockOpen"),
            BlockClose => write!(f, "BlockClose"),
            AttrIdentifier(str) => write!(f, "AttrIdentifier({str})"),
            AttrColon => write!(f, "AttrColon"),
            AttrSeparator => write!(f, "AttrSeparator"),
            ExprToken(expr) => write!(f, "ExpressionToken({expr})"),
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
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    String(String),
    Number(String),
    Operator(Operator),
    Var(String),
    VarDotAccessor,
    VarBracketAccessorOpen,
    VarBracketAccessorClose,
    FunctionArgsOpen,
    FunctionArgsClose,
    ParenthesesOpen,
    ParenthesesClose,
    ArrayOpen,
    ArrayClose,
}

impl Expr {
    fn len(&self) -> usize {
        match self {
            Expr::String(str) => str.len(),
            Expr::Number(str) => str.len(),
            Expr::Operator(op) => op.symbol().len(),
            Expr::Var(str) => str.len(),
            Expr::VarDotAccessor => 1,
            Expr::VarBracketAccessorOpen => 1,
            Expr::VarBracketAccessorClose => 1,
            Expr::FunctionArgsOpen => 1,
            Expr::FunctionArgsClose => 1,
            Expr::ParenthesesOpen => 1,
            Expr::ParenthesesClose => 1,
            Expr::ArrayOpen => 1,
            Expr::ArrayClose => 1,
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::String(str) => write!(f, "String({str})"),
            Expr::Number(str) => write!(f, "Number({str})"),
            Expr::Operator(op) => write!(f, "Operator({op})"),
            Expr::Var(str) => write!(f, "Variable({str})"),
            Expr::VarDotAccessor => write!(f, "VariableDotAccessor"),
            Expr::VarBracketAccessorOpen => write!(f, "VarBracketAccessorOpen"),
            Expr::VarBracketAccessorClose => write!(f, "VarBracketAccessorClose"),
            Expr::FunctionArgsOpen => write!(f, "FunctionArgsOpen"),
            Expr::FunctionArgsClose => write!(f, "FunctionArgsClose"),
            Expr::ParenthesesOpen => write!(f, "ParenthesesOpen"),
            Expr::ParenthesesClose => write!(f, "ParenthesesClose"),
            Expr::ArrayOpen => write!(f, "ArrayOpen"),
            Expr::ArrayClose => write!(f, "ArrayClose"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
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
            Operator::Plus,
            Operator::Minus,
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
            Operator::Plus => "+",
            Operator::Minus => "-",
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
            Operator::Plus => write!(f, "Add"),
            Operator::Minus => write!(f, "Sub"),
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
