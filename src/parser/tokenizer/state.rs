use crate::parser::tokenizer::State::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum State {
    Start,
    InLineComment,
    InBlockComment,

    ParsingIdentifier,
    ParsedBlockIdentifier,
    ParsedAttrIdentifier,

    ParsingBlock,
    // ResolvingIdentifierType,
    ParsingBlockClose,

    ParsingAttributeColon,
    ParsedAttrColon,
    ParsingAttrSeparator,
    ParsedAttrSeparator,

    ParsingEventListener,
    ParsingEventListenerIdentifier,
    ParsingEventListenerColon,
    ParsingEventListenerHandler,
    ParsedEventListener,

    ParsingDirective,
    ParsingDirectiveIdentifier,
    EmptyDirectiveParsed,
    ParsingDirectiveColon,
    ParsingDirectiveValue,
    ParsedDirective,

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
                ParsedBlockIdentifier => "ParsedBlockIdentifier",
                ParsedAttrIdentifier => "ParsedAttrIdentifier",
                ParsingBlock => "ParsingBlock",
                ParsingBlockClose => "ParsingBlockClose",
                ParsedAttrColon => "ParsedAttrColon",
                ParsingAttributeColon => "ParsingAttributeColon",
                ParsingAttrSeparator => "ParsingAttrSeparator",
                ParsedAttrSeparator => "ParsedAttrSeparator",
                ParsingEventListener => "ParsingEventListener",
                ParsedEventListener => "ParsedEventListener",
                ParsingEventListenerIdentifier => "ParsingEventListenerIdentifier",
                ParsingEventListenerColon => "ParsingEventListenerColon",
                ParsingEventListenerHandler => "ParsingEventListenerHandler",
                ParsingDirective => "ParsingDirective",
                ParsedDirective => "ParsedDirective",
                ParsingDirectiveIdentifier => "ParsingDirectiveIdentifier",
                EmptyDirectiveParsed => "EmptyDirectiveParsed",
                ParsingDirectiveColon => "ParsingDirectiveColon",
                ParsingDirectiveValue => "ParsingDirectiveValue",
                InDblQuoteUnescaped => "InDblQuoteUnescaped",
                InDblQuoteEscaped => "InDblQuoteEscaped",
            }
        )
    }
}
