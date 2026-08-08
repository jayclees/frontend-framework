use crate::parser::tokenizer::State::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum State {
    Start,
    InLineComment,
    InBlockComment,

    ParsingIdentifier,
    ResolvingIdentifier,
    ParsedBlockIdentifier,
    ParsedAttrIdentifier,

    ParsingBlockOpen,
    ParsedBlockOpen,
    ParsingBlockClose,

    ParsingAttributeColon,
    ParsedAttrColon,
    ParsingAttrSeparator,
    ParsedAttrSeparator,

    GatheringExpressionTokens,
    ParsedExpressionTokens,

    ParsingDirective,
    ParsingDirectiveOpen,
    ParsedDirectiveOpen,
    ParsingDirectiveIdentifier,
    ParsedDirectiveIdentifier,
    ParsedDirectiveColon,
    EmptyDirectiveParsed,
    ParsingDirectiveColon,
    ParsingDirectiveValue,
    ParsingDirectiveClose,
    ParsedDirectiveClose,
    ParsedDirective,

    ParsingEventListenerOpen,
    ParsedEventListenerOpen,
    ParsingEventListenerIdentifier,
    ParsingEventListenerColon,
    ParsedEventListenerColon,
    ParsingEventListenerHandler,
    ParsedEventListener,

    InDblQuoteUnescaped,
    InDblQuoteEscaped,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Start => "Start",
                InLineComment => "InLineComment",
                InBlockComment => "InBlockComment",

                ParsingIdentifier => "ParsingIdentifier",
                ResolvingIdentifier => "ResolvingIdentifier",
                ParsedBlockIdentifier => "ParsedBlockIdentifier",
                ParsedAttrIdentifier => "ParsedAttrIdentifier",

                ParsingBlockOpen => "ParsingBlockOpen",
                ParsedBlockOpen => "ParsedBlockOpen",
                ParsingBlockClose => "ParsingBlockClose",

                ParsingAttributeColon => "ParsingAttributeColon",
                ParsedAttrColon => "ParsedAttrColon",
                ParsingAttrSeparator => "ParsingAttrSeparator",
                ParsedAttrSeparator => "ParsedAttrSeparator",

                GatheringExpressionTokens => "GatheringExpressionTokens",
                ParsedExpressionTokens => "ParsedExpressionTokens",

                ParsingDirective => "ParsingDirective",
                ParsingDirectiveOpen => "ParsingDirectiveOpen",
                ParsedDirectiveOpen => "ParsedDirectiveOpen",
                ParsingDirectiveIdentifier => "ParsingDirectiveIdentifier",
                ParsedDirectiveIdentifier => "ParsedDirectiveIdentifier",
                ParsedDirectiveColon => "ParsedDirectiveColon",
                EmptyDirectiveParsed => "EmptyDirectiveParsed",
                ParsingDirectiveColon => "ParsingDirectiveColon",
                ParsingDirectiveValue => "ParsingDirectiveValue",
                ParsingDirectiveClose => "ParsingDirectiveClose",
                ParsedDirectiveClose => "ParsedDirectiveClose",
                ParsedDirective => "ParsedDirective",

                ParsingEventListenerOpen => "ParsingEventListenerOpen",
                ParsedEventListenerOpen => "ParsedEventListenerOpen",
                ParsingEventListenerIdentifier => "ParsingEventListenerIdentifier",
                ParsingEventListenerColon => "ParsingEventListenerColon",
                ParsedEventListenerColon => "ParsedEventListenerColon",
                ParsingEventListenerHandler => "ParsingEventListenerHandler",

                ParsedEventListener => "ParsedEventListener",
                InDblQuoteUnescaped => "InDblQuoteUnescaped",
                InDblQuoteEscaped => "InDblQuoteEscaped",
            }
        )
    }
}
