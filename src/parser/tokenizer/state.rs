use crate::parser::tokenizer::State::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum State {
    Start,
    ResolvingCommentType,
    ParsingLineComment,
    ParsingBlockComment,

    ParsingIdentifier,
    ResolvingIdentifier,
    ParsedBlockIdentifier,
    ParsedAttrIdentifier,

    ParsingBlock,
    ParsedAttrColon,

    ParsingDirective,
    ParsingDirectiveIdentifier,
    ParsedDirectiveIdentifier,
    ParsedDirectiveColon,
    ParsingDirectiveValue,

    ParsingEventListenerOpen,
    ParsedEventListenerOpen,
    ParsingEventListenerIdentifier,
    ParsedEventListenerIdentifier,
    ParsedEventListenerColon,
    ParsingEventListenerHandler,

    // Expressions
    ExprStart,
    ExprParsingNumber,
    ExprParsedNumber,
    ExprParsingString,
    ExprParsedString,
    ExprParsingVar,
    ExprParsedVar,
    ExprParsingOperator,
    ExprParsedOperator,
    ExprParsedBracketOpen,
    // ExprParsedFuncParenthesisOpen,
    // ExprParsedFuncOperand,
    ExprParsedParenthesisClose,
    ExprResolveSlash,

    // GatheringExpressionTokens,
    ParsedExpressionTokens,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Start => "Start",
                ResolvingCommentType => "ResolvingCommentType",
                ParsingLineComment => "ParsingLineComment",
                ParsingBlockComment => "ParsingBlockComment",

                ParsingIdentifier => "ParsingIdentifier",
                ResolvingIdentifier => "ResolvingIdentifier",
                ParsedBlockIdentifier => "ParsedBlockIdentifier",
                ParsedAttrIdentifier => "ParsedAttrIdentifier",

                ParsingBlock => "ParsingBlock",

                ParsedAttrColon => "ParsedAttrColon",

                ParsingDirective => "ParsingDirective",
                ParsingDirectiveIdentifier => "ParsingDirectiveIdentifier",
                ParsedDirectiveIdentifier => "ParsedDirectiveIdentifier",
                ParsedDirectiveColon => "ParsedDirectiveColon",
                ParsingDirectiveValue => "ParsingDirectiveValue",

                ParsingEventListenerOpen => "ParsingEventListenerOpen",
                ParsedEventListenerOpen => "ParsedEventListenerOpen",
                ParsingEventListenerIdentifier => "ParsingEventListenerIdentifier",
                ParsedEventListenerIdentifier => "ParsedEventListenerIdentifier",
                ParsedEventListenerColon => "ParsedEventListenerColon",
                ParsingEventListenerHandler => "ParsingEventListenerHandler",

                // Expressions
                ExprStart => "ExprStart",
                ExprParsingNumber => "ExprParsingNumber",
                ExprParsedNumber => "ExprParsedNumber",
                ExprParsingString => "ExprParsingString",
                ExprParsedString => "ExprParsedString",
                ExprParsingVar => "ExprParsingVar",
                ExprParsingOperator => "ExprParsingOperator",
                ExprParsedOperator => "ExprParsedOperator",
                ExprParsedBracketOpen => "ExprParsedBracketOpen",
                ExprParsedVar => "ExprParsedVar",
                ExprParsedParenthesisClose => "ExprParsedParenthesisClose",
                ExprResolveSlash => "ExprResolveSlash",

                ParsedExpressionTokens => "ParsedExpressionTokens",
            }
        )
    }
}
