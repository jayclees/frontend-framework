use crate::parser::tokenizer::directive::DirectiveEnum;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Token {
    start: usize,
    end: usize,
    r#type: TokenType,
}

impl Token {
    pub fn new(start: usize, r#type: TokenType) -> Token {
        // match and validate start/end with token len

        if r#type.len() == 0 {
            panic!(
                "Token should not be 0 length. TokenType: {}, cursor: {}",
                r#type, start
            )
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
        source_str == self.r#type.symbol()
    }
}

#[derive(Debug)]
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

    // For directive tokens
    ForDirectiveParenthesesOpen,
    ForDirectiveParenthesesClose,
    ForDirectiveInKeyword,
    ForDirectiveIndexComma,
}

impl TokenType {
    pub fn symbol(&self) -> &str {
        match self {
            TokenType::BlockIdentifier(str)
            | TokenType::LineComment(str)
            | TokenType::BlockComment(str)
            | TokenType::AttrIdentifier(str)
            | TokenType::DirectiveIdentifier(str)
            | TokenType::DirectiveValue(str)
            | TokenType::EventListenerHandler(str)
            | TokenType::EventListenerIdentifier(str) => str,

            TokenType::BlockOpen => "{",
            TokenType::BlockClose => "}",
            TokenType::AttrColon => ":",
            TokenType::AttrSeparator => ",",
            TokenType::DirectiveOpen => "[",
            TokenType::DirectiveClose => "]",
            TokenType::EventListenerOpen => "@[",
            TokenType::EventListenerClose => "]",
            TokenType::EventListenerColon => ":",
            TokenType::DirectiveColon => ":",
            TokenType::ForDirectiveParenthesesOpen => "(",
            TokenType::ForDirectiveParenthesesClose => ")",
            TokenType::ForDirectiveInKeyword => "in",
            TokenType::ForDirectiveIndexComma => ",",

            TokenType::ExprToken(expr) => expr.symbol(),
        }
    }

    pub fn len(&self) -> usize {
        self.symbol().len()
    }
    
    pub fn is_keyword(str: &str) -> KeywordResult {
        match str {
            "true" => KeywordResult::Expr(ExprToken::True),
            "false" => KeywordResult::Expr(ExprToken::False),
            "null" => KeywordResult::Expr(ExprToken::Null),
            "in" => KeywordResult::Expr(ExprToken::Null),
            _ => KeywordResult::None,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenType::BlockIdentifier(str) => write!(f, "BlockIdentifier({str})"),
            TokenType::LineComment(str) => write!(f, "LineComment({str})"),
            TokenType::BlockComment(str) => write!(f, "BlockComment({str})"),
            TokenType::BlockOpen => write!(f, "BlockOpen"),
            TokenType::BlockClose => write!(f, "BlockClose"),
            TokenType::AttrIdentifier(str) => write!(f, "AttrIdentifier({str})"),
            TokenType::AttrColon => write!(f, "AttrColon"),
            TokenType::AttrSeparator => write!(f, "AttrSeparator"),
            TokenType::ExprToken(expr) => write!(f, "ExpressionToken({expr})"),
            TokenType::DirectiveOpen => write!(f, "DirectiveOpen"),
            TokenType::DirectiveClose => write!(f, "DirectiveClose"),
            TokenType::DirectiveIdentifier(str) => write!(f, "DirectiveIdentifier({str})"),
            TokenType::DirectiveColon => write!(f, "DirectiveColon"),
            TokenType::DirectiveValue(str) => write!(f, "DirectiveValue({}, {})", "directive", str),
            TokenType::EventListenerOpen => write!(f, "EventListenerOpen"),
            TokenType::EventListenerClose => write!(f, "EventListenerClose"),
            TokenType::EventListenerIdentifier(str) => write!(f, "EventListenerIdentifier({str})"),
            TokenType::EventListenerColon => write!(f, "EventListenerColon"),
            TokenType::EventListenerHandler(str) => write!(f, "EventListenerHandler({str})"),
            TokenType::ForDirectiveParenthesesOpen => write!(f, "ForDirectiveParenthesesOpen"),
            TokenType::ForDirectiveParenthesesClose => write!(f, "ForDirectiveParenthesesClose"),
            TokenType::ForDirectiveInKeyword => write!(f, "ForDirectiveInKeyword"),
            TokenType::ForDirectiveIndexComma => write!(f, "ForDirectiveIndexComma"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprToken {
    String(String),
    Number(String),
    Operator(Operator),
    Var(String),
    PropAccessorVar(String),
    PropAccessorDot,
    UnresolvedDot,
    BracketAccessorOpen,
    BracketAccessorClose,
    FunctionArgsOpen,
    FunctionArgsClose,
    ArrayOpen,
    ArrayClose,
    True,
    False,
    Null,
}

impl ExprToken {
    // todo use these in tests to match tokens
    pub(crate) fn symbol(&self) -> &str {
        match self {
            ExprToken::String(str)
            | ExprToken::Number(str)
            | ExprToken::Var(str)
            | ExprToken::PropAccessorVar(str) => str,
            ExprToken::Operator(op) => op.symbol(),
            ExprToken::PropAccessorDot => ".",
            ExprToken::UnresolvedDot => ".",
            ExprToken::BracketAccessorOpen => "[",
            ExprToken::BracketAccessorClose => "]",
            ExprToken::FunctionArgsOpen => "(",
            ExprToken::FunctionArgsClose => ")",
            ExprToken::ArrayOpen => "[",
            ExprToken::ArrayClose => "]",
            ExprToken::True => "true",
            ExprToken::False => "false",
            ExprToken::Null => "null",
        }
    }

    fn len(&self) -> usize {
        self.symbol().len()
    }

    pub(crate) fn trim_quotes(&mut self) {
        match self {
            ExprToken::String(str) => {
                str.remove(0);
                str.pop();
            }
            _ => panic!("Should only call on string")
        }
    }
}

impl Display for ExprToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprToken::String(str) => write!(f, "String({str})"),
            ExprToken::Number(str) => write!(f, "Number({str})"),
            ExprToken::Operator(op) => write!(f, "Operator({op})"),
            ExprToken::Var(str) => write!(f, "Var({str})"),
            ExprToken::PropAccessorVar(str) => write!(f, "PropAccessorVar({str})"),
            ExprToken::PropAccessorDot => write!(f, "PropAccessorDot"),
            ExprToken::UnresolvedDot => write!(f, "UnresolvedDot"),
            ExprToken::BracketAccessorOpen => write!(f, "BracketAccessorOpen"),
            ExprToken::BracketAccessorClose => write!(f, "BracketAccessorClose"),
            ExprToken::FunctionArgsOpen => write!(f, "FunctionArgsOpen"),
            ExprToken::FunctionArgsClose => write!(f, "FunctionArgsClose"),
            ExprToken::ArrayOpen => write!(f, "ArrayOpen"),
            ExprToken::ArrayClose => write!(f, "ArrayClose"),
            ExprToken::True => write!(f, "True"),
            ExprToken::False => write!(f, "False"),
            ExprToken::Null => write!(f, "Null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Power, // ^x
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

    // Parentheses
    ParenthesesOpen,
    ParenthesesClose,

    // Logical
    And,
    Or,
    Not,
}

impl Operator {
    pub(super) fn is_operator_char(char: char) -> bool {
        ['+', '-', '*', '/', '%', '=', '<', '>', '!', '&', '|'].contains(&char)
    }

    pub(super) fn get_match(buf: &str) -> OperatorMatchResult {
        match buf {
            "+" => OperatorMatchResult::Matched(Operator::Plus),
            "-" => OperatorMatchResult::Matched(Operator::Minus),
            "*" => OperatorMatchResult::Matches(vec![Operator::Multiply, Operator::Power]),
            "**" => OperatorMatchResult::Matched(Operator::Power),
            "/" => OperatorMatchResult::Matched(Operator::Divide),
            "(" => OperatorMatchResult::Matched(Operator::ParenthesesOpen),
            ")" => OperatorMatchResult::Matched(Operator::ParenthesesClose),
            _ => OperatorMatchResult::None,
        }
    }

    pub(super) fn get_exact(buf: &str) -> Option<Operator> {
        match buf {
            "+" => Some(Operator::Plus),
            "-" => Some(Operator::Minus),
            "*" => Some(Operator::Multiply),
            "**" => Some(Operator::Power),
            "/" => Some(Operator::Divide),
            "(" => Some(Operator::ParenthesesOpen),
            ")" => Some(Operator::ParenthesesClose),
            _ => None,
        }
    }

    pub(super) fn symbol(&self) -> &'static str {
        match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "**",
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
            Operator::ParenthesesOpen => "(",
            Operator::ParenthesesClose => ")",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Not => "!",
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Operator::ParenthesesOpen => 1,
            Operator::Plus => 2,
            Operator::Minus => 2,
            Operator::Multiply => 3,
            Operator::Divide => 3,
            Operator::Modulus => 3,
            Operator::Power => 4,
            Operator::ParenthesesClose => unimplemented!(),
            Operator::Assign => unimplemented!(),
            Operator::Increment => unimplemented!(),
            Operator::Decrement => unimplemented!(),
            Operator::IncrementBy => unimplemented!(),
            Operator::DecrementBy => unimplemented!(),
            Operator::MultiplyBy => unimplemented!(),
            Operator::DivideBy => unimplemented!(),
            Operator::EqualTo => unimplemented!(),
            Operator::NotEqualTo => unimplemented!(),
            Operator::GreaterThan => unimplemented!(),
            Operator::GreaterThanOrEqualTo => unimplemented!(),
            Operator::LessThan => unimplemented!(),
            Operator::LessThanOrEqualTo => unimplemented!(),
            Operator::And => unimplemented!(),
            Operator::Or => unimplemented!(),
            Operator::Not => unimplemented!(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "Plus"),
            Operator::Minus => write!(f, "Minus"),
            Operator::Multiply => write!(f, "Multiply"),
            Operator::Divide => write!(f, "Divide"),
            Operator::Power => write!(f, "Power"),
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
            Operator::ParenthesesOpen => write!(f, "ParenthesesOpen"),
            Operator::ParenthesesClose => write!(f, "ParenthesesClose"),
            Operator::And => write!(f, "And"),
            Operator::Or => write!(f, "Or"),
            Operator::Not => write!(f, "Not"),
        }
    }
}

pub(super) enum OperatorMatchResult {
    Matched(Operator),
    Matches(Vec<Operator>),
    None,
}

pub(super) enum KeywordResult {
    Regular(TokenType),
    Expr(ExprToken),
    None,
}
