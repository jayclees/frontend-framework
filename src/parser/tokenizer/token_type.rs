use super::TokenType::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) r#type: TokenType,
}

impl Token {
    pub fn new(start: usize, end: usize, r#type: TokenType) -> Token {
        Token {
            start,
            end,
            r#type,
        }
    }
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
    ExprNumber(String),
    ExprVariable(String),
    ExprFunctionCall,
    ExprOperator(Operator),
    ExprParenthesesOpen,
    ExprParenthesesClose,
}

impl TokenType {
    pub fn len(&self) -> usize {
        match self {
            BlockIdentifier(str) |
            StringExpr(str) |
            LineComment(str) |
            BlockComment(str) |
            AttrIdentifier(str) |
            DirectiveIdentifier(str) |
            DirectiveValue(str) |
            EventListenerHandler(str) |
            ExprString(str) |
            ExprNumber(str) |
            ExprVariable(str) |
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
            ExprFunctionCall |
            ExprParenthesesOpen |
            ExprParenthesesClose => 1,

            EventListenerOpen  => 2,

            ExprOperator(_symbol) => {
                todo!()
            }
        }
    }
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
            ExprNumber(str) => write!(f, "ExprNumber({str})"),
            ExprVariable(str) => write!(f, "ExprVariable({str})"),
            ExprFunctionCall => write!(f, "ExprFunctionCall"),
            ExprOperator(symbol) => write!(f, "ExprOperator({symbol}"),
            ExprParenthesesOpen => write!(f, "ExprParenthesesOpen"),
            ExprParenthesesClose => write!(f, "ExprParenthesesClose"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
        match buf {
            "+" => OperatorMatchResult::Matched(Operator::Add),
            "-" => OperatorMatchResult::Matched(Operator::Sub),
            "*" => OperatorMatchResult::Matched(Operator::Multiply),
            "^" => OperatorMatchResult::Matched(Operator::Power),
            "/" => OperatorMatchResult::Matched(Operator::Divide),
            "%" => OperatorMatchResult::Matched(Operator::Modulus),
            "=" => OperatorMatchResult::Matched(Operator::Assign),
            "++" => OperatorMatchResult::Matched(Operator::Increment),
            "--" => OperatorMatchResult::Matched(Operator::Decrement),
            "+=" => OperatorMatchResult::Matched(Operator::IncrementBy),
            "-=" => OperatorMatchResult::Matched(Operator::DecrementBy),
            "*=" => OperatorMatchResult::Matched(Operator::MultiplyBy),
            "/=" => OperatorMatchResult::Matched(Operator::DivideBy),
            "==" => OperatorMatchResult::Matched(Operator::EqualTo),
            "!=" => OperatorMatchResult::Matched(Operator::NotEqualTo),
            ">" => OperatorMatchResult::Matched(Operator::GreaterThan),
            ">=" => OperatorMatchResult::Matched(Operator::GreaterThanOrEqualTo),
            "<" => OperatorMatchResult::Matched(Operator::LessThan),
            "<=" => OperatorMatchResult::Matched(Operator::LessThanOrEqualTo),
            "&&" => OperatorMatchResult::Matched(Operator::And),
            "||" => OperatorMatchResult::Matched(Operator::Or),
            "!" => OperatorMatchResult::Matched(Operator::Not),
            _ => OperatorMatchResult::Failed,
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
