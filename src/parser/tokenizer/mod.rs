mod expression;
mod helper;
mod state;
mod token_type;

use crate::parser::tokenizer::expression::{ExprResult, ExpressionTokenizer};
use crate::parser::tokenizer::helper::push_state_keep_buf;
use helper::{current, pop_state, prev, push_state, push_token, reset_buffer, TokenizerContract};
use state::State;
use state::State::*;
use std::cell::RefCell;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use token_type::{Token, TokenType, TokenType::*};

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
            r#"Error: {}. At line: {}, column: {}. State: "{}""#,
            msg,
            self.line,
            self.column,
            self.state()
        );
    }

    fn err_unexpected(&self, char: char) {
        // todo provide expected tokens based on current tokenizer state
        // dbg!(&self.tokens);
        // dbg!(&self.state_history);
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
                (prev.end, token.start)
            } else {
                (0, token.start)
            };
            let string = source.get_mut();
            let whitespace = string.take(ws_stop - ws_start).collect::<String>();
            let token_string = string.take(token.end - token.start).collect::<String>();

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
        let mut expr_tokenizer = ExpressionTokenizer::new();

        // todo:
        // Create method to attempt to recover from unexpected token and keep going? Recovery
        // method will depend on what state it is currently in and the state history.
        let mut peekable = string.chars().enumerate().peekable();
        for (cursor, char) in string.chars().enumerate() {
            self.cursor = cursor;

            if let Some((pos, _)) = peekable.peek()
                && pos <= &cursor
            {
                peekable.next();
            }

            if char == '\n' {
                self.line += 1;
                self.column = 0;
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
                        self.push_state(ParsingIdentifier, Some(char));
                    } else if char == '/' {
                        let next = peekable.peek();

                        if next.is_none() {
                            self.err_eof();
                        }

                        if next.unwrap().1 == '/' {
                            self.push_state(InLineComment, Some(char));
                        } else if next.unwrap().1 == '*' {
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
                ParsingIdentifier => {
                    // Parsing an identifier which could be either a block or attr identifier.
                    let is_in_block = self.prev_state().unwrap() == &ParsingBlock;

                    if self.buf.is_empty() && char.is_ascii_alphabetic() {
                        self.buf.push(char);
                    } else if !self.buf.is_empty() {
                        // Allow numbers, -, _ after first char in identifier.
                        if char.is_ascii_alphanumeric() || ['-', '_'].contains(&char) {
                            self.buf.push(char);
                        }
                        // Resolved to block identifier.
                        else if ['{', '['].contains(&char) {
                            self.push_token_pop_state(BlockIdentifier(self.buf.clone()));
                            self.push_state(ParsedBlockIdentifier, Some(char));
                        }
                        // Resolved to attr identifier.
                        else if [':', ',', '}'].contains(&char) {
                            if !is_in_block {
                                if char == '}' {
                                    self.err_unexpected(char);
                                } else {
                                    self.err_msg(
                                        "Attribute identifiers must be within blocks.".to_owned(),
                                    );
                                }
                            }
                            self.push_token_pop_state(AttrIdentifier(self.buf.clone()));
                            // if char == ':' {
                            //
                            // } else if char == ',' {
                            //
                            // } else if char == '}' {
                            //
                            // }
                            self.push_state(ParsedAttrIdentifier, Some(char));
                        }
                        // Peek the following chars, the next valid non whitespace char
                        // will decide whether this is a block or attr identifier.
                        else if char.is_whitespace() {
                            let mut i = 0;
                            loop {
                                i += 1;
                                if i > string.len() + 10 {
                                    panic!("infinite");
                                }
                                if let Some(next) = peekable.next() {
                                    if ['{', '['].contains(&next.1) {
                                        // Resolved to a block identifier.
                                        self.push_token_pop_state(BlockIdentifier(
                                            self.buf.clone(),
                                        ));
                                        self.push_state(ParsedBlockIdentifier, None);
                                        break;
                                    } else if [':', ','].contains(&next.1) {
                                        // Resolved to an attr identifier.
                                        if !is_in_block {
                                            self.err_msg(
                                                "Attribute identifiers must be within blocks."
                                                    .to_owned(),
                                            );
                                        }
                                        self.push_token_pop_state(AttrIdentifier(self.buf.clone()));
                                        self.push_state(ParsedAttrIdentifier, None);
                                        break;
                                    } else if next.1 == '}' {
                                        if !is_in_block {
                                            self.err_unexpected(next.1);
                                        }
                                        // Resolved to a boolean attribute identifier, and block is now closed
                                        self.push_token_pop_state(BlockIdentifier(
                                            self.buf.clone(),
                                        ));
                                        break;
                                    } else if !char.is_whitespace() {
                                        self.err_unexpected(next.1);
                                    }
                                } else {
                                    self.err_eof();
                                }
                            }
                        }
                    } else if !char.is_whitespace() {
                        // char must start with ascii alphabetic
                        self.err_unexpected(char);
                    }
                }
                ParsedBlockIdentifier => {
                    self.panic_if(
                        self.buf.len() > 1,
                        "Buffer should not be greater than 1 at this point".to_owned(),
                    );

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
                    if self.buf.as_str() == "{" {
                        self.push_token(BlockOpen);
                    }

                    if char.is_ascii_alphabetic() {
                        self.push_state(ParsingIdentifier, Some(char));
                    } else if char == '}' {
                        self.push_state(ParsingBlockClose, Some(char));
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char);
                    }
                }
                ParsingBlockClose => {
                    if self.buf.as_str() != "}" {
                        self.err_msg(
                            "Error internal implementation. Should only have } in buffer at this point."
                                .to_owned()
                        );
                    }

                    self.push_token_pop_state(BlockClose);
                }
                ParsedAttrIdentifier => {
                    let buf = self.buf.as_str();
                    if buf == ":" {
                        self.push_token_pop_state(AttrColon);
                        self.push_state(GatheringExpressionTokens, Some(char));
                    } else if buf == "," {
                        todo!()
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
                    self.push_state(GatheringExpressionTokens, Some(char));
                }
                ParsedAttrColon => {
                    self.pop_state();
                    if char.is_whitespace() {
                        self.push_state(GatheringExpressionTokens, None);
                    } else {
                        self.push_state(GatheringExpressionTokens, Some(char));
                    }
                    // expect expression
                    // todo create separate tokenizer for expressions?
                }
                ParsingAttrSeparator => {
                    // expect
                }
                ParsedAttrSeparator => {
                    crate::dd!(&self.buf);
                }
                GatheringExpressionTokens => {
                    let result = expr_tokenizer.push_parse(cursor, char);

                    match result {
                        ExprResult::StillParsing => {
                            // continue
                        }
                        ExprResult::Parsed(mut tokens) => {
                            self.tokens.append(&mut tokens);
                            self.pop_state();
                            self.push_state(ParsedExpressionTokens, Some(char));
                        }
                        ExprResult::Err(msg) => {
                            //
                        }
                        ExprResult::ErrUnexpected(msg) => {
                            eprintln!("{msg}");
                            self.err_unexpected(char);
                        }
                    }

                    // let last = self.buf.chars().last();
                    //
                    // // If a double quote was pushed onto buffer as
                    // // the state changed, mark in_dbl_quo = true.
                    // if let Some(last) = last
                    //     && self.buf.len() == 1
                    //     && last == '"'
                    // {
                    //     in_dbl_quo = true;
                    // }
                    //
                    // // Gather everything until comma or block close, then pass to expression tokenizer
                    // // todo Integrate the gathering into ExpressionTokenizer struct to avoid duplicating logic
                    // if !in_dbl_quo && (char == ',' || char == '}') {
                    //     // let mut expr_tokenizer = ExpressionTokenizer::new(cursor, &self.buf);
                    //     // let mut tokens = expr_tokenizer.tokenize();
                    //     // self.tokens.append(&mut tokens);
                    //     self.clear_buffer();
                    //     self.pop_state();
                    //     self.pop_state(); // Popping State::ParsedAttrColon
                    //     self.push_state(ParsedExpressionTokens, Some(char));
                    // } else if char == '"' {
                    //     if in_dbl_quo
                    //         && let Some(last) = last
                    //         && last != '\\'
                    //     {
                    //         in_dbl_quo = false;
                    //     } else {
                    //         in_dbl_quo = true;
                    //     }
                    //     self.buf.push(char);
                    // } else {
                    //     self.buf.push(char);
                    // }
                }
                ParsedExpressionTokens => match self.buf.as_str() {
                    "," => {
                        self.push_token_pop_state(AttrSeparator);
                        // if !char.is_whitespace() {
                        //     self.push_state(ParsedAttrSeparator, Some(char));
                        // } else {
                        //     self.push_state(ParsedAttrSeparator, None);
                        // }
                    }
                    "}" => {
                        self.push_token_pop_state(BlockClose);
                    }
                    _ => unimplemented!(),
                },
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

    fn push_token(&mut self, r#type: TokenType) {
        push_token(&mut self.tokens, r#type, &self.buf_start, self.cursor);
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
