mod expression;
mod helper;
mod state;
#[cfg(test)]
mod test;
pub(crate) mod token;

use helper::{
    TokenizerContract, current, pop_state, pop_state_type, prev, push_state, push_state_keep_buf,
    push_token, reset_buffer,
};
use state::State;
use state::State::*;
use std::cell::RefCell;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use token::{Token, TokenType};

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

    fn err_unexpected(&self, char: char) {
        // todo provide expected tokens based on current tokenizer state
        dbg!(&self.tokens);
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

    pub fn push_parse(&mut self, char: char, cursor: usize) -> TokenizerResult {
        self.cursor = cursor;

        if char == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        match self.state() {
            Start => {
                if char.is_ascii_alphabetic() {
                    self.push_state(ParsingIdentifier, Some(char));
                    return TokenizerResult::Parsing;
                }

                match char {
                    '{' => return TokenizerResult::Tokenized(vec![Token::resolved(cursor, TokenType::BlockOpen)]),
                    '}' => return TokenizerResult::Tokenized(vec![Token::resolved(cursor, TokenType::BlockClose)]),
                    _ => todo!(r#"handle "{}" char case"#, char)
                }
            }
            InLineComment => {}
            InBlockComment => {}
            ParsingIdentifier => {
                if char.is_ascii_alphabetic() {
                    self.buf.push(char);
                } else if char == ':' {
                    todo!()
                    // is attr identifier
                } else if char == '{' {
                    let token = Token::resolved(
                        self.buf_start.unwrap(),
                        TokenType::BlockIdentifier(self.buf.clone()),
                    );
                    let token2 = Token::resolved(self.cursor, TokenType::BlockOpen);
                    self.reset_buffer();
                    return TokenizerResult::Tokenized(vec![token, token2])
                } else if char == '(' {
                    // is fn identifier
                } else if char.is_whitespace() {
                    let token = Token::unresolved(self.buf_start.unwrap(), TokenType::UnresolvedIdentifier(self.buf.clone()));
                    self.pop_state();
                    self.reset_buffer();
                    return TokenizerResult::Tokenized(vec![token]);
                }
            }
            ResolvingIdentifier => {}
            ParsedBlockIdentifier => {}
            ParsedAttrIdentifier => {}
            ParsingBlockOpen => {}
            ParsingBlock => {}
            ParsingBlockClose => {}
            ParsingAttributeColon => {}
            ParsedAttrColon => {}
            ParsingAttrSeparator => {}
            ParsedAttrSeparator => {}
            GatheringExpressionTokens => {}
            ParsedExpressionTokens => {}
            ParsingDirective => {}
            ParsingDirectiveOpen => {}
            ParsedDirectiveOpen => {}
            ParsingDirectiveIdentifier => {}
            ParsedDirectiveIdentifier => {}
            ParsedDirectiveColon => {}
            EmptyDirectiveParsed => {}
            ParsingDirectiveColon => {}
            ParsingDirectiveValue => {}
            ParsingDirectiveClose => {}
            ParsedDirective => {}
            ParsingEventListenerOpen => {}
            ParsedEventListenerOpen => {}
            ParsingEventListenerIdentifier => {}
            ParsingEventListenerColon => {}
            ParsedEventListenerColon => {}
            ParsingEventListenerHandler => {}
            ParsedEventListener => {}
            ParsedDirectiveOrEventClose => {}
            InDblQuoteUnescaped => {}
            InDblQuoteEscaped => {}
        }

        // All states should have been popped, and we should be back to start
        // if self.state_history.len() != 1 && self.state() != &Start {
        //     self.err_eof();
        // }

        // self.match_with_source();

        TokenizerResult::Parsing
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

pub enum TokenizerResult {
    Parsing,
    Tokenized(Vec<Token>),
    Err(String),
}
