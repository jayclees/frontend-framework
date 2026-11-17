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
    ParsingDirectiveValue,
    DirectiveExprStart,
    ExpectDirectiveClose,

    ParsingEventListenerOpen,
    ParsedEventListenerOpen,
    ParsingEventListenerIdentifier,
    ParsedEventListenerIdentifier,
    ParsedEventListenerColon,
    ParsingEventListenerHandler,

    // Expressions
    ExprStart,
    ExprParsingNumber,
    ExprParsingFractionalNumber,
    ExprParsedNumber,
    ExprExpectStringStart,
    ExprParsingString,
    ExprParsedString,
    ExprParsingVar,
    ExprParsedVar,
    ExprParsingOperator,
    ExprParsedOperator,
    ExprParsedBracketOpen,
    ExprParsedParenthesisClose,
    ExprResolvingSlash,
    ExprResolvingDot,
    ExprParsedNegativeOperator,

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
                ParsingDirectiveValue => "ParsingDirectiveValue",
                DirectiveExprStart => "DirectiveExprStart",
                ExpectDirectiveClose => "ExpectDirectiveClose",
                ParsingEventListenerOpen => "ParsingEventListenerOpen",
                ParsedEventListenerOpen => "ParsedEventListenerOpen",
                ParsingEventListenerIdentifier => "ParsingEventListenerIdentifier",
                ParsedEventListenerIdentifier => "ParsedEventListenerIdentifier",
                ParsedEventListenerColon => "ParsedEventListenerColon",
                ParsingEventListenerHandler => "ParsingEventListenerHandler",
                // Expressions
                ExprStart => "ExprStart",
                ExprParsingNumber => "ExprParsingNumber",
                ExprParsingFractionalNumber => "ExprParsingFractionalNumber",
                ExprParsedNumber => "ExprParsedNumber",
                ExprExpectStringStart => "ExprExpectStringStart",
                ExprParsingString => "ExprParsingString",
                ExprParsedString => "ExprParsedString",
                ExprParsingVar => "ExprParsingVar",
                ExprParsingOperator => "ExprParsingOperator",
                ExprParsedOperator => "ExprParsedOperator",
                ExprParsedBracketOpen => "ExprParsedBracketOpen",
                ExprParsedVar => "ExprParsedVar",
                ExprParsedParenthesisClose => "ExprParsedParenthesisClose",
                ExprResolvingSlash => "ExprResolvingSlash",
                ExprResolvingDot => "ExprResolvingDot",
                ExprParsedNegativeOperator => "ExprParsedNegativeOperator",
                ParsedExpressionTokens => "ParsedExpressionTokens",
            }
        )
    }
}
