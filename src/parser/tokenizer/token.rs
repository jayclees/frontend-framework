use super::TokenType::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
    start: usize,
    end: usize,
    r#type: TokenType,
    resolved: bool,
}

impl Token {
    pub fn resolved(start: usize, r#type: TokenType) -> Token {
        if r#type.len() == 0 {
            panic!("Token should not be 0 length. TokenType: {}, cursor: {}", r#type, start)
        }

        Token {
            start,
            end: start + r#type.len(),
            r#type,
            resolved: true
        }
    }
    pub fn unresolved(start: usize, r#type: TokenType) -> Token {
        if r#type.len() == 0 {
            panic!("Token should not be 0 length. TokenType: {}, cursor: {}", r#type, start)
        }

        Token {
            start,
            end: start + r#type.len(),
            r#type,
            resolved: false
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
    
    pub fn set_type(&mut self, r#type: TokenType) {
        self.r#type = r#type;
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved
    }

    #[cfg(test)]
    pub fn test(&self, source: &str) -> bool {
        let source_str = &source[self.start..self.end];
        match &self.r#type {
            UnresolvedIdentifier(str) |
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
            BlockOrAttrSeparator => source_str == ",",
            DirectiveOpen => source_str == "[",
            DirectiveClose => source_str == "]",
            DirectiveColon => source_str == ":",
            EventListenerOpen => source_str == "@[",
            EventListenerClose => source_str == "]",
            EventListenerColon => source_str == ":",

            ExprToken(expr) => match expr {
                ExprToken::String(str) => source_str == str,
                ExprToken::Number(str) => source_str == str,
                ExprToken::Operator(op) => source_str == op.symbol(),
                ExprToken::Var(str) => source_str == str,
                ExprToken::VarDotAccessor => source_str == ".",
                ExprToken::VarBracketAccessorOpen => source_str == "[",
                ExprToken::VarBracketAccessorClose => source_str == "]",
                ExprToken::FunctionArgsOpen => source_str == "(",
                ExprToken::FunctionArgsClose => source_str == ")",
                ExprToken::ParenthesesOpen => source_str == "(",
                ExprToken::ParenthesesClose => source_str == ")",
                ExprToken::ArrayOpen => source_str == "[",
                ExprToken::ArrayClose => source_str == "]",
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keyword(KeywordEnum),
    UnresolvedIdentifier(String), // This can be block id, attr id
    BlockIdentifier(String),
    LineComment(String),
    BlockComment(String),
    BlockOpen,
    BlockClose,
    AttrIdentifier(String),
    AttrColon,
    BlockOrAttrSeparator,
    ExprToken(ExprToken),
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
            UnresolvedIdentifier(str) |
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
            BlockOrAttrSeparator |
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
            UnresolvedIdentifier(str) => write!(f, "UnresolvedIdentifier({str})"),
            BlockIdentifier(str) => write!(f, "BlockIdentifier({str})"),
            LineComment(str) => write!(f, "LineComment({str})"),
            BlockComment(str) => write!(f, "BlockComment({str})"),
            BlockOpen => write!(f, "BlockOpen"),
            BlockClose => write!(f, "BlockClose"),
            AttrIdentifier(str) => write!(f, "AttrIdentifier({str})"),
            AttrColon => write!(f, "AttrColon"),
            BlockOrAttrSeparator => write!(f, "BlockOrAttrSeparator"),
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
pub enum ExprToken {
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

impl ExprToken {
    fn len(&self) -> usize {
        match self {
            ExprToken::String(str) => str.len(),
            ExprToken::Number(str) => str.len(),
            ExprToken::Operator(op) => op.symbol().len(),
            ExprToken::Var(str) => str.len(),
            ExprToken::VarDotAccessor => 1,
            ExprToken::VarBracketAccessorOpen => 1,
            ExprToken::VarBracketAccessorClose => 1,
            ExprToken::FunctionArgsOpen => 1,
            ExprToken::FunctionArgsClose => 1,
            ExprToken::ParenthesesOpen => 1,
            ExprToken::ParenthesesClose => 1,
            ExprToken::ArrayOpen => 1,
            ExprToken::ArrayClose => 1,
        }
    }
}

impl Display for ExprToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprToken::String(str) => write!(f, "String({str})"),
            ExprToken::Number(str) => write!(f, "Number({str})"),
            ExprToken::Operator(op) => write!(f, "Operator({op})"),
            ExprToken::Var(str) => write!(f, "Variable({str})"),
            ExprToken::VarDotAccessor => write!(f, "VariableDotAccessor"),
            ExprToken::VarBracketAccessorOpen => write!(f, "VarBracketAccessorOpen"),
            ExprToken::VarBracketAccessorClose => write!(f, "VarBracketAccessorClose"),
            ExprToken::FunctionArgsOpen => write!(f, "FunctionArgsOpen"),
            ExprToken::FunctionArgsClose => write!(f, "FunctionArgsClose"),
            ExprToken::ParenthesesOpen => write!(f, "ParenthesesOpen"),
            ExprToken::ParenthesesClose => write!(f, "ParenthesesClose"),
            ExprToken::ArrayOpen => write!(f, "ArrayOpen"),
            ExprToken::ArrayClose => write!(f, "ArrayClose"),
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
