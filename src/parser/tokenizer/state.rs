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
    
    ParsingForDirectiveValue,

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
    ExprParsingVarOrKeyword,
    ExprParsedVarOrKeyword,
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
                ParsingForDirectiveValue => "ParsingForDirectiveValue",
                ParsingEventListenerOpen => "ParsingEventListenerOpen",
                ParsedEventListenerOpen => "ParsedEventListenerOpen",
                ParsingEventListenerIdentifier => "ParsingEventListenerIdentifier",
                ParsedEventListenerIdentifier => "ParsedEventListenerIdentifier",
                ParsedEventListenerColon => "ParsedEventListenerColon",
                ParsingEventListenerHandler => "ParsingEventListenerHandler",
                ExprStart => "ExprStart",
                ExprParsingNumber => "ExprParsingNumber",
                ExprParsingFractionalNumber => "ExprParsingFractionalNumber",
                ExprParsedNumber => "ExprParsedNumber",
                ExprExpectStringStart => "ExprExpectStringStart",
                ExprParsingString => "ExprParsingString",
                ExprParsedString => "ExprParsedString",
                ExprParsingVarOrKeyword => "ExprParsingVarOrKeyword",
                ExprParsedVarOrKeyword => "ExprParsedVarOrKeyword",
                ExprParsingOperator => "ExprParsingOperator",
                ExprParsedOperator => "ExprParsedOperator",
                ExprParsedBracketOpen => "ExprParsedBracketOpen",
                ExprParsedParenthesisClose => "ExprParsedParenthesisClose",
                ExprResolvingSlash => "ExprResolvingSlash",
                ExprResolvingDot => "ExprResolvingDot",
                ExprParsedNegativeOperator => "ExprParsedNegativeOperator",
                ParsedExpressionTokens => "ParsedExpressionTokens",
            }
        )
    }
}
