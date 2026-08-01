mod state;
mod token_type;

use super::tokenizer::TokenType::*;
use regex::Regex;
use state::State;
use state::State::*;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use token_type::Token;

#[derive(Debug)]
pub struct Tokenizer {
    state_history: Vec<State>,
    cursor: usize,
    line: usize,
    column: usize,
    buf: String,
    buf_start: Option<usize>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            state_history: vec![Start],
            cursor: 0,
            line: 1,
            column: 0,
            buf: String::new(),
            buf_start: None,
            tokens: vec![],
        }
    }
}

impl Tokenizer {
    fn state(&self) -> &State {
        self.state_history.last().unwrap()
    }

    fn push_state(&mut self, state: State, starting_char: Option<char>) {
        self.state_history.push(state);
        self.buf.clear();
        self.buf_start = Some(self.cursor);

        if let Some(char) = starting_char {
            self.buf.push(char);
        }
    }

    fn pop_state(&mut self) {
        // self.clear_buffer();
        if !self.buf.is_empty() {
            // An uncleared buffer may indicate something is wrong somewhere, so panic.
            dbg!(&self.buf);
            panic!("Buffer should be already empty here?");
        }
        self.state_history.pop();
    }

    fn push_token(&mut self, r#type: TokenType) {
        let start = self.buf_start.expect("buf_start should be set.");
        let end = self.cursor;

        if start == end {
            dbg!(r#type, start, end);
            panic!("Token should not be zero length.");
        }

        self.tokens.push(Token { start, end, r#type });
        self.clear_buffer();
    }

    fn push_token_pop_state(&mut self, r#type: TokenType) {
        self.push_token(r#type);
        self.pop_state();
    }

    fn clear_buffer(&mut self) {
        self.buf.clear();
        self.buf_start = Some(self.cursor);
    }

    fn err_msg(&self, msg: String) {
        panic!(
            r#"Error: {} at line: {}, column: {}. State: "{}""#,
            msg,
            self.line,
            self.column,
            self.state()
        );
    }

    fn err_unexpected(&self, char: char) {
        // todo provide expected tokens based on current tokenizer state
        dbg!(&self.tokens);
        panic!(
            r#"Unexpected token "{}" at line: {}, column: {}. State: "{}""#,
            char,
            self.line,
            self.column,
            self.state()
        );
    }

    fn err_eof(&self) {
        panic!("Unexpected end of file.");
    }

    fn match_with_source(&self) {
        let source = read_to_string("app/my-page.app").unwrap();
        let mut source = RefCell::new(source.chars());
        let mut highlighted = String::new();
        let mut prev: Option<&Token> = None;
        for token in self.tokens.iter().clone() {
            let (ws_start, ws_stop) = if let Some(prev) = prev {
                (prev.end, token.start)
            } else {
                (0, token.start)
            };
            let string = source.get_mut();
            let whitespace = string.take(ws_stop - ws_start).collect::<String>();
            let token_string = string.take(token.end - token.start).collect::<String>();
            // match token.r#type {
            //     BlockIdentifier(_) => println!(r#"BlockIdentifier: '{x}'"#),
            //     AttrIdentifier(_) => println!(r#"AttrIdentifier: '{x}'"#),
            //     StringExpr(_) => println!(r#"StringExpr: '{x}'"#),
            //     LineComment(_) => println!(r#"LineComment: '{x}'"#),
            //     BlockComment(_) => println!(r#"BlockComment: '{x}'"#),
            //     Colon => println!(r#"Colon: '{x}'"#),
            //     BlockOpen => println!(r#"BlockOpen: '{x}'"#),
            //     BlockClose => println!(r#"BlockClose: '{x}'"#),
            //     Expression => println!(r#"Expression: '{x}'"#),
            //     DirectiveOpen => println!(r#"DirectiveOpen: '{x}'"#),
            //     DirectiveClose => println!(r#"DirectiveClose: '{x}'"#),
            //     DirectiveIdentifier(_) => println!(r#"DirectiveIdentifier: '{x}'"#),
            //     DirectiveColon => println!(r#"DirectiveColon: '{x}'"#),
            //     DirectiveValue(_) => println!(r#"DirectiveValue: '{x}'"#),
            // }

            let token_type = token.r#type.to_string();
            let display = if let Some(i) = token_type.find("(") {
                token_type[..i].to_owned()
            } else {
                token_type
            };
            highlighted.push_str(
                format!(
                    r#"{}<span class="{}">{}</span>"#,
                    whitespace,
                    display,
                    html_escape::encode_text(&token_string).as_ref()
                )
                .as_str(),
            );
            prev = Some(token);
        }

        // append remaining untokenized text
        highlighted.push_str(
            format!(
                r#"<span class="Untokenized">{}</span>"#,
                html_escape::encode_text(source.get_mut().as_str())
            )
            .as_str(),
        );

        let layout = read_to_string("generated/layout.html")
            .unwrap()
            .replace("{REPLACE}", highlighted.as_str());
        let bytes = layout.as_bytes();
        let written = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("generated/generated.html")
            .unwrap()
            .write(bytes);

        if let Ok(written) = written {
            if written == bytes.len() {
                println!("{written} bytes written to generated/generated.html");
            } else {
                eprintln!(
                    "Failed to successfully write to generated/generated.html. {}/{} bytes written.",
                    bytes.len(),
                    written
                );
            }
        } else {
            dbg!("Failed to write to generated/generated.html");
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        let string = read_to_string("app/my-page.app").unwrap();
        let mut skip: usize = 0;
        let _block_id_reg = Regex::new(r#"\A[A-Z][a-zA-Z]\z"#).unwrap();
        let _attr_id_reg = Regex::new(r#"\A[A-Z][a-zA-Z]\z"#).unwrap();
        let _newline_reg = Regex::new(r#"\n"#).unwrap();

        // todo Create method to attempt to recover from unexpected token and keep going? Recovery
        // todo method will depend on what state it is currently in and the state history.
        let mut peekable = string.chars().peekable();
        for (cursor, char) in string.chars().enumerate() {
            peekable.next();
            self.cursor = cursor;

            if char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            if skip > 0 {
                skip -= 1;
                continue;
            }

            match self.state() {
                Start => {
                    if char.is_ascii_alphabetic() {
                        self.push_state(ParsingBlockIdentifier, Some(char));
                    } else if char == '/' {
                        let next = peekable.peek();

                        if next.is_none() {
                            self.err_eof();
                        }

                        if next.unwrap() == &'/' {
                            self.push_state(InLineComment, Some(char));
                        } else if next.unwrap() == &'*' {
                            self.push_state(InBlockComment, Some(char));
                        } else {
                            self.err_unexpected(char);
                        }
                    } else if char == '#' {
                        self.push_state(InLineComment, Some(char));
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char);
                    } else {
                        // ignore
                    }
                }
                InLineComment => {
                    // ignore everything until new line
                    if char == '\n' {
                        self.push_token_pop_state(LineComment(self.buf.clone()));
                    } else {
                        self.buf.push(char);
                    }
                }
                InBlockComment => {
                    if self.buf.ends_with("*/") {
                        self.push_token_pop_state(BlockComment(self.buf.clone()));
                    } else {
                        self.buf.push(char);
                    }
                }
                ParsingBlockIdentifier => {
                    // allow numbers once first letter is identified as an ascii letter
                    if char.is_ascii_alphanumeric() {
                        self.buf.push(char);
                    } else if char.is_whitespace() || char == '[' || char == '{' {
                        self.push_token_pop_state(BlockIdentifier(self.buf.clone()));
                        if !char.is_whitespace() {
                            self.push_state(ParsedBlockIdentifier, Some(char));
                        } else {
                            self.push_state(ParsedBlockIdentifier, None);
                        }
                    } else {
                        self.err_unexpected(char);
                    }
                    // match char against regex
                    // terminate on whitespace or curly, append token
                }
                ParsedBlockIdentifier => {
                    // First start state, push token after to fix
                    // off by one error (cursor was off by -1).
                    if !char.is_whitespace() {
                        match char {
                            '[' => self.push_state(ParsingDirective, Some(char)),
                            '{' => self.push_state(ParsingBlock, Some(char)),
                            '@' => self.push_state(ParsingEventListener, Some(char)),
                            _ => self.err_unexpected(char),
                        }
                    }
                }
                ParsingBlock => {
                    self.push_token(BlockOpen);
                    if char.is_ascii_alphabetic() {
                        self.push_state(ParsingAttrIdentifier, Some(char));
                    } else if char.is_whitespace() {
                        self.push_state(ParsingAttrIdentifier, None);
                    } else {
                        self.err_unexpected(char);
                    }
                }
                ParsingBlockClose => {
                    if self.buf.as_str() != "}" {
                        self.err_msg("".to_owned());
                    }
                }
                ParsingAttrIdentifier => {
                    if self.buf.is_empty() {
                        if char.is_alphabetic() {
                            self.buf.push(char);
                        } else if char.is_ascii_digit() {
                            self.err_msg(
                                "Attribute identifiers must start with alphabetic character."
                                    .to_owned(),
                            );
                        } else if !char.is_whitespace() {
                            self.err_unexpected(char);
                        }
                    } else {
                        if char.is_ascii_alphanumeric() {
                            self.buf.push(char);
                        } else if char.is_whitespace() || [':', ',', '}'].contains(&char) {
                            self.push_token_pop_state(AttrIdentifier(self.buf.clone()));

                            if char.is_whitespace() {
                                // could still be colon, comma, or close curly
                                self.push_state(ParsedAttrIdentifier, None);
                            } else {
                                match char {
                                    ':' => self.push_state(ParsingAttributeColon, Some(char)),
                                    ',' => self.push_state(ParsingAttrSeparator, Some(char)),
                                    '}' => self.push_state(ParsingBlockClose, Some(char)),
                                    _ => unimplemented!(),
                                }
                            }
                        } else {
                            self.err_unexpected(char);
                        }
                    }
                    // if terminated on comma or block close curly, assume
                    // an attribute like disabled in `<input disabled>`
                }
                ParsedAttrIdentifier => {
                    if char == ':' {
                        self.buf.push(char);
                        self.push_state(ParsingAttributeColon, Some(char));
                    }
                    // was just terminated by whitespace, colon, or comma
                    // expect another attr id, or block id, or close curly
                }
                ParsingAttributeColon => {
                    if self.buf.as_str() != ":" {
                        dbg!(&self.buf);
                        self.err_msg(
                            "Error internal implementation. Should only have : in buffer at this point.".to_owned()
                        );
                    }

                    self.push_token_pop_state(AttrColon);
                    self.push_state(ParsedAttrColon, Some(char));
                }
                ParsedAttrColon => {
                    // expect expression
                    // todo create separate tokenizer for expressions?
                }
                ParsingAttrSeparator => {
                    // expect
                }
                ParsedAttrSeparator => {
                    //
                }
                ParsingEventListener => {
                    let buf = self.buf.as_str();
                    if buf != "@" && buf != "@[" {
                        dbg!(&buf);
                        self.err_msg(
                            "Error internal implementation. Should only have @ or @[ in buffer at this point.".to_owned()
                        );
                    }

                    if char == '[' {
                        self.buf.push(char);
                    } else if buf == "@[" {
                        self.push_token(EventListenerOpen);

                        if char.is_ascii_alphabetic() {
                            self.push_state(ParsingEventListenerIdentifier, Some(char));
                        } else {
                            self.err_unexpected(char);
                        }
                    } else {
                        self.err_unexpected(char);
                    }
                }
                ParsingEventListenerIdentifier => {
                    // This state should be entered with ascii_alphabetic as first char
                    if char.is_ascii_alphabetic() || char == '.' {
                        self.buf.push(char);
                    } else if char == ':' {
                        self.push_token_pop_state(EventListenerIdentifier(self.buf.clone()));
                        self.push_state(ParsingEventListenerColon, Some(char));
                    } else {
                        self.err_unexpected(char);
                    }
                }
                ParsingEventListenerColon => {
                    if self.buf.as_str() != ":" {
                        dbg!(&self.buf);
                        panic!(
                            "Error internal implementation. Should only have : in buffer at this point."
                        );
                    }
                    self.push_token_pop_state(EventListenerColon);
                    self.push_state(ParsingEventListenerHandler, Some(char));
                }
                ParsingEventListenerHandler => {
                    // todo for now just put everything into this until bracket close
                    if char == ']' {
                        self.push_token_pop_state(EventListenerHandler(self.buf.clone()));
                        self.push_state(ParsedEventListener, Some(char));
                    } else {
                        self.buf.push(char);
                    }
                }
                ParsedEventListener => {
                    self.push_token(EventListenerClose);
                    self.pop_state();
                    self.pop_state(); // Popping State::ParsingEventListener
                }
                ParsingDirective => {
                    if !char.is_whitespace() && char.is_ascii_alphabetic() {
                        self.push_token(DirectiveOpen);
                        self.push_state(ParsingDirectiveIdentifier, Some(char));
                    } else if char == '-' {
                        self.err_msg(r#"Directive identifiers cannot start with '-'."#.to_owned());
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char);
                    }
                }
                ParsingDirectiveIdentifier => {
                    if char.is_ascii_alphabetic() || char == '-' {
                        self.buf.push(char);
                    } else if char == ':' {
                        self.push_token_pop_state(DirectiveIdentifier(self.buf.clone()));
                        self.push_state(ParsingDirectiveColon, Some(char));
                    } else if char == ']' {
                        self.push_token_pop_state(DirectiveIdentifier(self.buf.clone()));
                        self.push_state(EmptyDirectiveParsed, None);
                    }
                }
                EmptyDirectiveParsed => {
                    self.push_token_pop_state(DirectiveClose);
                    self.pop_state(); // Popping State::ParsingDirective
                }
                ParsingDirectiveColon => {
                    if self.buf.as_str() != ":" {
                        dbg!(&self.buf);
                        panic!(
                            "Error internal implementation. Should only have : in buffer at this point."
                        );
                    }

                    self.push_token_pop_state(DirectiveColon);
                    self.push_state(ParsingDirectiveValue, None);
                }
                ParsingDirectiveValue => {
                    // for now just grab everything until close bracket
                    if char == ']' {
                        self.push_token_pop_state(DirectiveValue(self.buf.clone()));
                        self.push_state(ParsedDirective, Some(char));
                    } else {
                        self.buf.push(char);
                    }

                    // potentially create another tokenizer for directive values, parse based
                    // on directive identifier. e.g. [if: expr], [for post in posts]
                    // or create a generic (simple) expression tokenizer
                }
                ParsedDirective => {
                    if self.buf.as_str() != "]" {
                        dbg!(&self.buf);
                        panic!(
                            "Error internal implementation. Should only have : in buffer at this point."
                        );
                    }

                    self.push_token_pop_state(DirectiveClose);
                    self.pop_state(); // Popping State::ParsingDirective
                }
                InDblQuoteUnescaped => {
                    // if char is dbl quote, exit current state into X state
                    // if char is backslash, enter into InDblQuoteEscaped
                }
                InDblQuoteEscaped => {
                    // if char is backslash again, enter into InDblQuoteUnescaped
                }
            }
        }

        // All states should have been popped, and we should be back to start
        // if self.state() != &Start {
        //     self.err_eof();
        // }

        self.match_with_source();

        &self.tokens
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
        }
    }
}
