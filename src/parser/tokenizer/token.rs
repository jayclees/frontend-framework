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
        match &self.r#type {
            TokenType::BlockIdentifier(str)
            | TokenType::LineComment(str)
            | TokenType::BlockComment(str)
            | TokenType::AttrIdentifier(str)
            | TokenType::DirectiveIdentifier(str)
            | TokenType::EventListenerIdentifier(str)
            | TokenType::EventListenerHandler(str)
            | TokenType::DirectiveValue(str) => source_str == str,

            TokenType::BlockOpen => source_str == "{",
            TokenType::BlockClose => source_str == "}",
            TokenType::AttrColon => source_str == ":",
            TokenType::AttrSeparator => source_str == ",",
            TokenType::DirectiveOpen => source_str == "[",
            TokenType::DirectiveClose => source_str == "]",
            TokenType::DirectiveColon => source_str == ":",
            TokenType::EventListenerOpen => source_str == "@[",
            TokenType::EventListenerClose => source_str == "]",
            TokenType::EventListenerColon => source_str == ":",

            TokenType::ExprToken(expr) => match expr {
                ExprToken::String(str) => source_str == str,
                ExprToken::Number(str) => source_str == str,
                ExprToken::Operator(op) => source_str == op.symbol(),
                ExprToken::Var(str) => source_str == str,
                ExprToken::Dot => source_str == ".",
                ExprToken::BracketAccessorOpen => source_str == "[",
                ExprToken::BracketAccessorClose => source_str == "]",
                ExprToken::FunctionArgsOpen => source_str == "(",
                ExprToken::FunctionArgsClose => source_str == ")",
                ExprToken::ArrayOpen => source_str == "[",
                ExprToken::ArrayClose => source_str == "]",
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
            TokenType::BlockIdentifier(str)
            | TokenType::LineComment(str)
            | TokenType::BlockComment(str)
            | TokenType::AttrIdentifier(str)
            | TokenType::DirectiveIdentifier(str)
            | TokenType::DirectiveValue(str)
            | TokenType::EventListenerHandler(str)
            | TokenType::EventListenerIdentifier(str) => str.len(),

            TokenType::BlockOpen
            | TokenType::BlockClose
            | TokenType::AttrColon
            | TokenType::AttrSeparator
            | TokenType::DirectiveOpen
            | TokenType::DirectiveClose
            | TokenType::EventListenerClose
            | TokenType::EventListenerColon
            | TokenType::DirectiveColon => 1,

            TokenType::EventListenerOpen => 2,

            TokenType::ExprToken(expr) => expr.len(),
        }
    }

    pub fn dump(&self) -> String {
        match self {
            TokenType::BlockIdentifier(str) |
            TokenType::LineComment(str) |
            TokenType::BlockComment(str) |
            TokenType::AttrIdentifier(str) |
            TokenType::DirectiveIdentifier(str) |
            TokenType::DirectiveValue(str) |
            TokenType::EventListenerIdentifier(str) |
            TokenType::EventListenerHandler(str) => str.to_owned(),

            TokenType::BlockOpen => "{".to_owned(),
            TokenType::BlockClose => "}".to_owned(),
            TokenType::AttrColon => ":".to_owned(),
            TokenType::AttrSeparator => ",".to_owned(),
            TokenType::DirectiveOpen => "[".to_owned(),
            TokenType::DirectiveClose => "]".to_owned(),
            TokenType::DirectiveColon => ":".to_owned(),
            TokenType::EventListenerOpen => "@[".to_owned(),
            TokenType::EventListenerClose => "]".to_owned(),
            TokenType::EventListenerColon => ":".to_owned(),

            TokenType::ExprToken(expr_tok) => match expr_tok {
                ExprToken::String(str) |
                ExprToken::Number(str) |
                ExprToken::Var(str)  => str.to_owned(),
                ExprToken::Operator(op) => op.symbol().to_owned(),
                ExprToken::Dot => ".".to_owned(),
                ExprToken::BracketAccessorOpen => "[".to_owned(),
                ExprToken::BracketAccessorClose => "]".to_owned(),
                ExprToken::FunctionArgsOpen => "(".to_owned(),
                ExprToken::FunctionArgsClose => ")".to_owned(),
                ExprToken::ArrayOpen => "[".to_owned(),
                ExprToken::ArrayClose => "]".to_owned(),
            }
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
            TokenType::DirectiveIdentifier(_) => write!(f, "DirectiveIdentifier"),
            TokenType::DirectiveColon => write!(f, "DirectiveColon"),
            TokenType::DirectiveValue(str) => write!(f, "DirectiveValue({str})"),
            TokenType::EventListenerOpen => write!(f, "EventListenerOpen"),
            TokenType::EventListenerClose => write!(f, "EventListenerClose"),
            TokenType::EventListenerIdentifier(str) => write!(f, "EventListenerIdentifier({str})"),
            TokenType::EventListenerColon => write!(f, "EventListenerColon"),
            TokenType::EventListenerHandler(str) => write!(f, "EventListenerHandler({str})"),
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
    Dot,
    BracketAccessorOpen,
    BracketAccessorClose,
    FunctionArgsOpen,
    FunctionArgsClose,
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
            ExprToken::Dot => 1,
            ExprToken::BracketAccessorOpen => 1,
            ExprToken::BracketAccessorClose => 1,
            ExprToken::FunctionArgsOpen => 1,
            ExprToken::FunctionArgsClose => 1,
            ExprToken::ArrayOpen => 1,
            ExprToken::ArrayClose => 1,
        }
    }

    pub(crate) fn add(&self, right: ExprToken) -> String {
        if let ExprToken::Number(right) = right {
            match self {
                ExprToken::Number(left) => {
                    if left.contains(".") || right.contains(".") {
                        (left.parse::<f64>().unwrap() + right.parse::<f64>().unwrap()).to_string()
                    } else {
                        (left.parse::<usize>().unwrap() + right.parse::<usize>().unwrap())
                            .to_string()
                    }
                }
                token => {
                    panic!(r#""{token}" is not a number."#)
                }
            }
        } else {
            panic!("Right hand operand is not a number.")
        }
    }

    pub(crate) fn subtract(&self, right: ExprToken) -> String {
        if let ExprToken::Number(right) = right {
            match self {
                ExprToken::Number(left) => {
                    if left.contains(".") || right.contains(".") {
                        (left.parse::<f64>().unwrap() - right.parse::<f64>().unwrap()).to_string()
                    } else {
                        (left.parse::<isize>().unwrap() - right.parse::<isize>().unwrap())
                            .to_string()
                    }
                }
                token => {
                    panic!(r#""{token}" is not a number."#)
                }
            }
        } else {
            panic!("Right hand operand is not a number.")
        }
    }

    pub(crate) fn multiply(&self, right: ExprToken) -> String {
        if let ExprToken::Number(right) = right {
            match self {
                ExprToken::Number(left) => {
                    if left.contains(".") || right.contains(".") {
                        (left.parse::<f64>().unwrap() * right.parse::<f64>().unwrap()).to_string()
                    } else {
                        (left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap())
                            .to_string()
                    }
                }
                token => {
                    panic!(r#""{token}" is not a number."#)
                }
            }
        } else {
            panic!("Right hand operand is not a number.")
        }
    }

    pub(crate) fn divide(&self, right: ExprToken) -> String {
        if let ExprToken::Number(right) = right {
            match self {
                ExprToken::Number(left) => {
                    (left.parse::<f64>().unwrap() / right.parse::<f64>().unwrap()).to_string()
                }
                token => {
                    panic!(r#""{token}" is not a number."#)
                }
            }
        } else {
            panic!("Right hand operand is not a number.")
        }
    }

    pub(crate) fn power(&self, right: ExprToken) -> String {
        if let ExprToken::Number(right) = right {
            match self {
                ExprToken::Number(left) => {
                    if left.contains(".") || right.contains(".") {
                        left.parse::<f64>()
                            .unwrap()
                            .powf(right.parse::<f64>().unwrap())
                            .to_string()
                    } else {
                        match left
                            .parse::<usize>()
                            .unwrap()
                            .checked_pow(right.parse::<u32>().unwrap())
                        {
                            Some(result) => result.to_string(),
                            None => panic!("Failed raising {left} to the power of {right}."),
                        }
                    }
                }
                token => {
                    panic!(r#""{token}" is not a number."#)
                }
            }
        } else {
            panic!("Right hand operand is not a number.")
        }
    }

    pub(crate) fn modulus(&self, right: ExprToken) -> String {
        if let ExprToken::Number(right) = right {
            match self {
                ExprToken::Number(left) => {
                    if left.contains(".") || right.contains(".") {
                        (left.parse::<f64>().unwrap() % right.parse::<f64>().unwrap()).to_string()
                    } else {
                        (left.parse::<usize>().unwrap() % right.parse::<usize>().unwrap())
                            .to_string()
                    }
                }
                token => {
                    panic!(r#""{token}" is not a number."#)
                }
            }
        } else {
            panic!("Right hand operand is not a number.")
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
            ExprToken::Dot => write!(f, "Dot"),
            ExprToken::BracketAccessorOpen => write!(f, "BracketAccessorOpen"),
            ExprToken::BracketAccessorClose => write!(f, "BracketAccessorClose"),
            ExprToken::FunctionArgsOpen => write!(f, "FunctionArgsOpen"),
            ExprToken::FunctionArgsClose => write!(f, "FunctionArgsClose"),
            ExprToken::ArrayOpen => write!(f, "ArrayOpen"),
            ExprToken::ArrayClose => write!(f, "ArrayClose"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // String
    Concatenation, // .

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
            Operator::Concatenation => ".",
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
            Operator::Concatenation => 5,
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
            Operator::Concatenation => write!(f, "Concatenation"),
        }
    }
}

pub(super) enum OperatorMatchResult {
    Matched(Operator),
    Matches(Vec<Operator>),
    None,
}
