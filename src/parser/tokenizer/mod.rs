mod expression;
mod helper;
mod state;
#[cfg(test)]
mod test;
pub(crate) mod token;

use expression::{ExprResult, ExpressionTokenizer};
use helper::{
    TokenizerContract, current, pop_state, pop_state_type, prev, push_state, push_state_keep_buf,
    push_token, reset_buffer,
};
use state::State;
use state::State::*;
use std::cell::RefCell;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use token::{Token, TokenType, TokenType::*};

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
    fn err_msg(&self, msg: String) {
        let msg = msg.trim_end_matches(".");
        panic!(
            r#"{}. At line: {}, column: {}. State: "{}""#,
            msg,
            self.line,
            self.column,
            self.state()
        );
    }

    fn err_unexpected(&self, char: char, msg: Option<String>) {
        // todo provide expected tokens based on current tokenizer state
        // dbg!(&self.tokens);
        // dbg!(&self.state_history);
        panic!(
            r#"Unexpected token "{}" at line: {}, column: {}. State: "{}"{}"#,
            char,
            self.line,
            self.column,
            self.state(),
            match msg {
                Some(msg) => {
                    format!(". {msg}")
                }
                None => "".to_owned(),
            }
        );
    }

    fn err_eof(&self) {
        eprintln!("Unexpected end of file. State history:");
        dbg!(&self.tokens);
        dbg!(&self.state_history);
        panic!("Unexpected end of file.");
    }

    fn panic_if(&self, condition: bool, msg: String) {
        if condition {
            panic!(
                r#"{}. At line: {}, column: {}. State: "{}""#,
                msg,
                self.line,
                self.column,
                self.state()
            );
        }
    }

    fn match_with_source(&self) {
        let source = read_to_string("app/my-page.app").unwrap();
        let mut source = RefCell::new(source.chars());
        let mut highlighted = String::new();
        let mut prev: Option<&Token> = None;
        for token in self.tokens.iter().clone() {
            let (ws_start, ws_stop) = if let Some(prev) = prev {
                (prev.end(), token.start())
            } else {
                (0, token.start())
            };
            let string = source.get_mut();
            let whitespace = string.take(ws_stop - ws_start).collect::<String>();
            let token_string = string.take(token.end() - token.start()).collect::<String>();

            let token_type = token.r#type().to_string();
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

    fn expect_identifier_start(&mut self, char: char) -> TokenizerResult {
        if char.is_ascii_alphabetic() {
            self.reset_buffer();
            self.buf.push(char);
            self.push_state_keep_buf(ParsingIdentifier, None);
            return TokenizerResult::Handled;
        }

        TokenizerResult::NextPipe
    }

    fn parse_identifier_char(&mut self, char: char) -> TokenizerResult {
        if char.is_ascii_alphabetic() {
            self.buf.push(char);
            TokenizerResult::Handled
        } else if char == ':' {
            if self.prev_state().expect("Should have state") != &ParsingBlock {
                eprintln!("Cannot have attribute identifier outside of block.");
                self.err_unexpected(char, None);
            }
            self.push_token_pop_state(AttrIdentifier(self.buf.clone()));
            self.buf.push(char);
            self.push_token(AttrColon);
            TokenizerResult::PushState(GatheringExpressionTokens)
        } else if char.is_whitespace() {
            TokenizerResult::PushStateKeepBuf(ResolvingIdentifier, None)
        } else {
            TokenizerResult::ErrUnexpected
        }
    }

    fn resolve_identifier_type(&mut self, char: char) -> TokenizerResult {
        if char == '[' || char == '@' || char == '{' {
            // Resolved to block identifier
            self.push_token_pop_state(BlockIdentifier(self.buf.clone()));
            self.pop_state_type(ParsingIdentifier);
            return TokenizerResult::PushStateKeepBuf(ParsedBlockIdentifier, Some(char));
        } else if char == ':' || char == ',' {
            // Resolved to attr identifier
            self.push_token_pop_state(AttrIdentifier(self.buf.clone()));
            self.pop_state_type(ParsingIdentifier);
            if char == ':' {
                self.reset_buffer();
                self.push_token(AttrColon);
                return TokenizerResult::PushState(GatheringExpressionTokens);
            } else if char == ',' {
                self.reset_buffer();
                self.push_token(AttrSeparator);
                return TokenizerResult::PushState(GatheringExpressionTokens);
            }
        } else if !char.is_whitespace() {
            return TokenizerResult::ErrUnexpected;
        }

        TokenizerResult::Handled
    }

    fn pipeline(&mut self, char: char, pipes: Vec<Pipe>) {
        for pipe in pipes {
            let result = match pipe {
                Pipe::ExpectIdentifierStart => self.expect_identifier_start(char),
                Pipe::ParseIdentifierChar => self.parse_identifier_char(char),
                Pipe::ResolveIdentifierType => self.resolve_identifier_type(char),
                // Pipe::ExpectCommentStart => self.expect_comment_start(char),
            };
            match result {
                TokenizerResult::Handled => return,
                TokenizerResult::NextPipe | TokenizerResult::Void => { /* Do nothing */ }
                TokenizerResult::ReplaceState(next) => {
                    self.pop_state();
                    self.push_state(next, None);
                    return;
                }
                TokenizerResult::PushState(next) => return self.push_state(next, None),
                TokenizerResult::PushStateKeepBuf(next, char) => {
                    return self.push_state_keep_buf(next, char);
                }
                TokenizerResult::PopState => {}
                TokenizerResult::PopStateType(_) => {}
                TokenizerResult::Err => {}
                TokenizerResult::ErrUnexpected => self.err_unexpected(char, None),
            };
        }
    }

    pub fn tokenize(&mut self, string: String) -> &Vec<Token> {
        let mut expr_tokenizer = ExpressionTokenizer::new();

        // todo:
        // Create method to attempt to recover from unexpected token and keep going? Recovery
        // method will depend on what state it is currently in and the state history.
        for (cursor, char) in string.chars().enumerate() {
            self.cursor = cursor;

            if char == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            match self.state() {
                Start => self.pipeline(
                    char,
                    vec![
                        Pipe::ExpectIdentifierStart,
                        // Pipe::ExpectCommentStart
                    ],
                ),
                InLineComment => todo!("InLineComment"),
                InBlockComment => todo!("InBlockComment"),
                ParsingIdentifier => self.pipeline(char, vec![Pipe::ParseIdentifierChar]),
                ResolvingIdentifier => self.pipeline(char, vec![Pipe::ResolveIdentifierType]),
                ParsedBlockIdentifier => {
                    if self.buf.is_empty() {
                        if matches!(char, '[' | '@' | '{') {
                            self.reset_buffer();
                            self.buf.push(char);
                        } else if !char.is_whitespace() {
                            self.err_unexpected(char, None);
                        }
                    } else {
                        if self.buf.as_str() == "[" {
                            self.push_token(DirectiveOpen);
                            if char.is_whitespace() {
                                self.push_state(ParsedDirectiveOpen, None);
                            } else {
                                self.push_state(ParsingDirectiveIdentifier, Some(char));
                            }
                        } else if self.buf.as_str() == "@" {
                            if char == '[' {
                                self.buf.push(char);
                                self.push_token(EventListenerOpen);
                                self.push_state(ParsingEventListenerIdentifier, None);
                            } else {
                                self.err_unexpected(char, None);
                            }
                        } else if self.buf.as_str() == "{" {
                            self.push_token(BlockOpen);
                            self.pop_state_type(ParsedBlockIdentifier);
                            self.push_state(ParsingBlock, None);
                            if char.is_ascii_alphabetic() {
                                self.push_state(ParsingIdentifier, Some(char));
                            } else if char == '}' {
                                self.buf.push(char);
                                self.push_token(BlockClose);
                                self.pop_state_type(ParsingBlock);
                            }
                        }
                    }
                }
                ParsedAttrIdentifier => todo!("ParsedAttrIdentifier"),
                ParsingBlockOpen => todo!("ParsingBlockOpen"),
                ParsingBlock => {
                    if char.is_ascii_alphabetic() {
                        self.push_state(ParsingIdentifier, Some(char));
                    } else if char == '}' {
                        self.reset_buffer();
                        self.buf.push(char);
                        self.push_token_pop_state(BlockClose);
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char, None);
                    }
                }
                ParsingBlockClose => todo!("ParsingBlockClose"),
                ParsingAttributeColon => todo!("ParsingAttributeColon"),
                ParsedAttrColon => todo!("ParsedAttrColon"),
                ParsingAttrSeparator => todo!("ParsingAttrSeparator"),
                ParsedAttrSeparator => todo!("ParsedAttrSeparator"),
                GatheringExpressionTokens => {
                    match expr_tokenizer.push_parse(cursor, char) {
                        ExprResult::StillParsing => {
                            // continue
                        }
                        ExprResult::Parsed(mut tokens) => {
                            self.tokens.append(&mut tokens);
                            self.pop_state();
                            if char == '}' {
                                self.reset_buffer();
                                self.push_token(BlockClose);
                                self.pop_state_type(ParsingBlock);
                            } else if char == ',' {
                                self.reset_buffer();
                                self.push_token(AttrSeparator);
                            } else {
                                self.push_state(ParsedExpressionTokens, Some(char));
                            }
                        }
                        ExprResult::Err(msg) => {
                            self.err_msg(msg);
                        }
                        ExprResult::ErrExpectedExprStart(msg) => {
                            self.err_unexpected(char, Some(msg));
                        }
                        ExprResult::ErrUnexpected(expr_state) => {
                            self.err_unexpected(
                                char,
                                Some(format!("Expr tokenizer state: {expr_state}")),
                            );
                        }
                        ExprResult::ParsedBlockIdentifier(_) => {}
                    }
                }
                ParsedExpressionTokens => todo!("ParsedExpressionTokens"),
                ParsingDirective => todo!("ParsingDirective"),
                ParsingDirectiveOpen => todo!("ParsingDirectiveOpen"),
                ParsedDirectiveOpen => {
                    if char.is_ascii_alphabetic() {
                        self.pop_state();
                        self.push_state(ParsingDirectiveIdentifier, Some(char));
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char, None);
                    }
                }
                ParsingDirectiveIdentifier => {
                    let buf_empty = self.buf.is_empty();
                    if buf_empty {
                        self.reset_buffer();
                    }
                    if buf_empty && char.is_ascii_alphabetic() {
                        self.buf.push(char);
                    } else if !buf_empty
                        && (char.is_ascii_alphabetic() || ['-', '_'].contains(&char))
                    {
                        self.buf.push(char)
                    } else if char == ':' {
                        self.push_token_pop_state(DirectiveIdentifier(self.buf.clone()));
                        self.buf.push(char);
                        self.push_token(DirectiveColon);
                        self.push_state(ParsingDirectiveValue, None);
                    } else if char == ']' {
                        self.push_token(DirectiveIdentifier(self.buf.clone()));
                        self.buf.push(char);
                        self.push_token(DirectiveClose);
                        self.pop_state();
                    } else if char.is_whitespace() {
                        self.push_token_pop_state(DirectiveIdentifier(self.buf.clone()));
                        self.push_state(ParsedDirectiveIdentifier, None);
                    } else {
                        self.err_unexpected(char, None);
                    }
                }
                ParsedDirectiveIdentifier => {
                    if char == ':' {
                        self.reset_buffer();
                        self.buf.push(char);
                        self.push_token_pop_state(DirectiveColon);
                        self.push_state(ParsingDirectiveValue, None);
                    } else if char == ']' {
                        self.reset_buffer();
                        self.buf.push(char);
                        self.push_token_pop_state(DirectiveClose);
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char, None);
                    }
                }
                ParsedDirectiveColon => todo!("ParsedDirectiveColon"),
                EmptyDirectiveParsed => todo!("EmptyDirectiveParsed"),
                ParsingDirectiveColon => {
                    if self.buf.is_empty() {
                        self.reset_buffer();
                    }
                    if self.buf.as_str() == ":" {
                        self.push_token_pop_state(DirectiveColon);
                        self.push_state(ParsingDirectiveValue, None);
                    } else if char == ':' {
                        self.buf.push(char);
                    } else {
                        self.err_unexpected(char, None);
                    }
                }
                ParsingDirectiveValue => {
                    if self.buf.is_empty() {
                        self.reset_buffer();
                    }

                    if char == ']' {
                        self.push_token_pop_state(DirectiveValue(self.buf.clone()));
                        self.buf.push(char);
                        self.push_token(DirectiveClose);
                        self.push_state(ParsedDirectiveOrEventClose, None);
                    } else {
                        self.buf.push(char);
                    }
                }
                ParsingDirectiveClose => todo!("ParsingDirectiveClose"),
                ParsedDirective => todo!("ParsedDirective"),
                ParsingEventListenerOpen => todo!("ParsingEventListenerOpen"),
                ParsedEventListenerOpen => todo!("ParsedEventListenerOpen"),
                ParsingEventListenerIdentifier => match self.buf.is_empty() {
                    true => {
                        self.reset_buffer();
                        if char.is_ascii_alphabetic() {
                            self.buf.push(char);
                        } else if !char.is_whitespace() {
                            self.err_unexpected(char, None);
                        }
                    }
                    false => {
                        if char.is_ascii_alphanumeric() {
                            self.buf.push(char);
                        } else if char == ':' {
                            self.push_token_pop_state(EventListenerIdentifier(self.buf.clone()));
                            self.buf.push(char);
                            self.push_token(EventListenerColon);
                            self.push_state(ParsingEventListenerHandler, None);
                        } else if char.is_whitespace() {
                            self.push_token_pop_state(EventListenerIdentifier(self.buf.clone()));
                            self.push_state(ParsingEventListenerColon, None);
                        } else {
                            self.err_unexpected(char, None);
                        }
                    }
                },
                ParsingEventListenerColon => {
                    if char == ':' {
                        self.reset_buffer();
                        self.buf.push(char);
                        self.push_token_pop_state(EventListenerColon);
                        self.push_state(ParsingEventListenerHandler, None);
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char, None);
                    }
                }
                ParsedEventListenerColon => todo!("ParsedEventListenerColon"),
                ParsingEventListenerHandler => {
                    if self.buf.is_empty() {
                        self.reset_buffer();
                    }
                    // push everything to buf and end on close bracket for now
                    if char == ']' {
                        self.push_token_pop_state(EventListenerHandler(self.buf.clone()));
                        self.buf.push(char);
                        self.push_token(EventListenerClose);
                        self.push_state(ParsedDirectiveOrEventClose, None);
                    } else {
                        self.buf.push(char);
                    }
                }
                ParsedEventListener => todo!("ParsedEventListener"),
                ParsedDirectiveOrEventClose => {
                    // just make sure the first char is whitespace
                    if char.is_whitespace() {
                        self.pop_state();
                    } else {
                        self.err_unexpected(char, None);
                    }
                }
                InDblQuoteUnescaped => todo!("InDblQuoteUnescaped"),
                InDblQuoteEscaped => todo!("InDblQuoteEscaped"),
            }
        }

        // All states should have been popped, and we should be back to start
        if self.state_history.len() != 1 && self.state() != &Start {
            self.err_eof();
        }

        // self.match_with_source();

        &self.tokens
    }
}

impl TokenizerContract for Tokenizer {
    type TokenizerState = State;

    fn state(&self) -> &State {
        current(&self.state_history)
    }

    fn prev_state(&self) -> Option<&State> {
        prev(&self.state_history)
    }

    fn push_state(&mut self, state: State, starting_char: Option<char>) {
        push_state(
            &mut self.state_history,
            state,
            &mut self.buf,
            &mut self.buf_start,
            starting_char,
            self.cursor,
        );
    }

    fn push_state_keep_buf(&mut self, state: Self::TokenizerState, starting_char: Option<char>) {
        push_state_keep_buf(&mut self.state_history, state, &mut self.buf, starting_char);
    }

    fn pop_state(&mut self) {
        pop_state(&mut self.state_history);
    }

    fn pop_state_type(&mut self, state: Self::TokenizerState) {
        pop_state_type(&mut self.state_history, state);
    }

    fn push_token(&mut self, r#type: TokenType) {
        let buf_start = self.buf_start.expect("Should be set.");
        push_token(&mut self.tokens, buf_start, r#type);
        self.reset_buffer();
    }

    fn push_token_pop_state(&mut self, r#type: TokenType) {
        self.push_token(r#type);
        self.pop_state();
    }

    fn buf_last(&self) -> Option<char> {
        self.buf.chars().last()
    }

    fn reset_buffer(&mut self) {
        reset_buffer(&mut self.buf, &mut self.buf_start, self.cursor);
    }
}

enum TokenizerResult {
    Handled,
    NextPipe,
    ReplaceState(State),
    PushState(State),
    PushStateKeepBuf(State, Option<char>),
    PopState,
    PopStateType(State),
    Err,
    ErrUnexpected,
    Void,
}

enum Pipe {
    ExpectIdentifierStart,
    ParseIdentifierChar, // ExpectCommentStart,
    ResolveIdentifierType,
}
