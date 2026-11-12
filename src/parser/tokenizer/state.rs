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

    // ParsingBlockOpen,
    ParsingBlock,
    // ParsingBlockClose,

    // ParsingAttributeColon,
    ParsedAttrColon,
    // ParsingAttrSeparator,
    // ParsedAttrSeparator,

    GatheringExpressionTokens,
    ParsedExpressionTokens,

    ParsingDirective,
    // ParsingDirectiveOpen,
    // ParsedDirectiveOpen,
    ParsingDirectiveIdentifier,
    ParsedDirectiveIdentifier,
    ParsedDirectiveColon,
    // EmptyDirectiveParsed,
    // ParsingDirectiveColon,
    ParsingDirectiveValue,
    // ParsingDirectiveClose,
    // ParsedDirective,

    ParsingEventListenerOpen,
    ParsedEventListenerOpen,
    ParsingEventListenerIdentifier,
    // ParsingEventListenerColon,
    ParsedEventListenerColon,
    ParsingEventListenerHandler,
    // ParsedEventListener,

    // ParsedDirectiveOrEventClose,

    // InDblQuoteUnescaped,
    // InDblQuoteEscaped,
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

                GatheringExpressionTokens => "GatheringExpressionTokens",
                ParsedExpressionTokens => "ParsedExpressionTokens",

                ParsingDirective => "ParsingDirective",
                ParsingDirectiveIdentifier => "ParsingDirectiveIdentifier",
                ParsedDirectiveIdentifier => "ParsedDirectiveIdentifier",
                ParsedDirectiveColon => "ParsedDirectiveColon",
                ParsingDirectiveValue => "ParsingDirectiveValue",

                ParsingEventListenerOpen => "ParsingEventListenerOpen",
                ParsedEventListenerOpen => "ParsedEventListenerOpen",
                ParsingEventListenerIdentifier => "ParsingEventListenerIdentifier",
                ParsedEventListenerColon => "ParsedEventListenerColon",
                ParsingEventListenerHandler => "ParsingEventListenerHandler",
            }
        )
    }
}
