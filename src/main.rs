mod helper;
mod tokenizer;

use std::cell::RefCell;
use crate::State::*;
use crate::TokenType::*;
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::process::exit;

#[derive(Debug)]
struct Tokenizer {
    state_history: Vec<State>,
    cursor: usize,
    line: usize,
    column: usize,
    buf: String,
    buf_start: Option<usize>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn state(&self) -> &State {
        self.state_history.last().unwrap()
    }

    pub fn start_state(&mut self, state: State, starting_char: Option<char>) {
        self.state_history.push(state);
        if let Some(char) = starting_char {
            self.buf.push(char);
            self.buf_start = Some(self.cursor);
        }
    }

    pub fn pop_state(&mut self) {
        self.buf.clear();
        self.buf_start = Some(self.cursor);
        self.state_history.pop();
    }

    pub fn push_token(&mut self, r#type: TokenType) {
        let start = self.buf_start.expect("buf_start should be set.");
        let end = self.cursor;

        // if start == end {
        //     dbg!(r#type, start, end);
        //     panic!("Token should not be zero length.");
        // }

        self.tokens.push(Token { start, end, r#type });
    }

    pub fn push_token_pop_state(&mut self, r#type: TokenType) {
        self.push_token(r#type);
        self.pop_state();
    }

    fn err_unexpected(&self, char: char) {
        dbg!(&self.tokens);
        panic!(
            r#"Unexpected token "{}" at line: {}, column: {}."#,
            char, self.line, self.column
        );
    }

    fn err_eof(&self) {
        panic!("Unexpected end of file.");
    }

    fn match_with_source(&self) {
        let string = read_to_string("app/my-page.app").unwrap();
        let mut string = RefCell::new(string.chars());
        let mut highlighted = String::new();
        let mut prev: Option<&Token> = None;
        for token in self.tokens.iter().clone() {
            let skip = if let Some(prev) = prev {
                token.start - prev.end
            } else {
                0
            };
            let x = string.get_mut().skip(skip).take(token.end - token.start).collect::<String>();
            match token.r#type {
                BlockIdentifier(_) => println!(r#"BlockIdentifier: "{x}""#),
                AttrIdentifier(_) => println!(r#"AttrIdentifier: "{x}""#),
                StringExpr(_) => println!(r#"StringExpr: "{x}""#),
                LineComment(_) => println!(r#"LineComment: "{x}""#),
                BlockComment(_) => println!(r#"BlockComment: "{x}""#),
                Colon => println!(r#"Colon: "{x}""#),
                OpenCurly => println!(r#"OpenCurly: "{x}""#),
                CloseCurly => println!(r#"CloseCurly: "{x}""#),
                Expression => println!(r#"Expression: "{x}""#),
                DirectiveOpen => println!(r#"DirectiveOpen: "{x}""#),
                DirectiveClose => println!(r#"DirectiveClose: "{x}""#),
                DirectiveIdentifier(_) => println!(r#"DirectiveIdentifier: "{x}""#),
                DirectiveColon => println!(r#"DirectiveColon: "{x}""#),
            }

            // highlighted.push_str(format!("<span>{x}</span>").as_str());
            prev = Some(token);
        }

        // println!("{highlighted}");
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
                        self.start_state(ParsingBlockIdentifier, Some(char));
                    } else if char == '/' {
                        let next = peekable.peek();

                        if next.is_none() {
                            self.err_eof();
                        }

                        if next.unwrap() == &'/' {
                            self.start_state(InLineComment, Some(char));
                        } else if next.unwrap() == &'*' {
                            self.start_state(InBlockComment, Some(char));
                        } else {
                            self.err_unexpected(char);
                        }
                    } else if char == '#' {
                        self.start_state(InLineComment, Some(char));
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char);
                    } else {
                        // ignore
                    }
                }
                DefaultState => {
                    //
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
                        // todo this is wrong. we need to check the next char in this char iteration
                        // and push token/pop state.
                    } else if char.is_whitespace() || char == '[' || char == '{' {
                        self.push_token_pop_state(BlockIdentifier(self.buf.clone()));
                        self.start_state(ParsedBlockIdentifier, Some(char));
                    } else {
                        self.err_unexpected(char);
                    }
                    // match char against regex
                    // terminate on whitespace or curly, append token
                }
                ParsedBlockIdentifier => {
                    if !char.is_whitespace() && char == '{' {
                        self.push_token_pop_state(OpenCurly);
                        self.start_state(InBlock, None);
                    } else if !char.is_whitespace() && char == '[' {
                        self.push_token(DirectiveOpen);
                        self.start_state(InDirective, None);
                        self.buf.clear();
                    } else if !char.is_whitespace() && char == '@' {
                        let next = string.chars().nth(cursor + 1);

                        if next.is_none() {
                            self.err_eof();
                        }

                        if next.unwrap() == '[' {
                            self.start_state(ParsingEventListener, None);
                            skip += 1; // the next char will be [, which we can skip
                        } else {
                            self.err_unexpected(char);
                        }
                    } else {
                        self.err_unexpected(char);
                    }
                }
                InBlock => {
                    // expect attr id or block id
                    if char.is_ascii_alphabetic() {
                        self.start_state(ParsingIdentifier, Some(char));
                    }
                }
                ParsingIdentifier => {
                    //
                }
                ParsingAttrIdentifier => {
                    // terminate if whitespace or colon
                    // if terminated on whitespace, assume an attribute like disabled in `<input disabled>`
                }
                ParsedAttrIdentifier => {
                    // was just terminated by whitespace or colon
                    // expect another attr id, or block id, or close curly
                }
                ParsingEventListener => {
                    // match against event identifier regex
                    // terminate on colon
                }
                ParsedEventListener => {
                    // expect function call definition or some kind of expression that does something to the app state
                }
                InDirective => {
                    if !char.is_whitespace() && char.is_ascii_alphabetic() {
                        self.start_state(ParsingDirectiveIdentifier, Some(char));
                    } else if !char.is_whitespace() {
                        self.err_unexpected(char);
                    }
                }
                ParsingDirectiveIdentifier => {
                    if char.is_ascii_alphabetic() {
                        self.buf.push(char);

                        // let next = peekable.next().expect("Unexpected end of file.");
                        // if !next.is_ascii_alphabetic() {
                        //     self.push_token_pop_state(DirectiveIdentifier(self.buf.clone()));
                        //     self.start_state(ParsedDirectiveIdentifier, None);
                        // }
                    } else if char == ':' {
                        self.push_token_pop_state(DirectiveIdentifier(self.buf.clone()));
                        self.push_token(DirectiveColon);
                        self.start_state(ParsedDirectiveIdentifier, None);
                        // self.push_token_pop_state()
                    } else if char == ']' {

                    }
                }
                ParsedDirectiveIdentifier => {
                    // directives might be blank?
                    if char == ':' {
                        self.push_token(DirectiveColon);
                        // dbg!(&self.tokens);
                        // dbg!(&self.buf);
                        // std::process::exit(1);
                        // start state based on directive type? or buffer everything until
                        // bracket close and hand off to directive value tokenizer?
                    } else if char == ']' {
                        //
                    }
                    // expect function call definition or some kind of expression that does something to the app state
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

        self.match_with_source();

        &self.tokens
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Start,
    DefaultState,
    InLineComment,
    InBlockComment,

    ParsingBlockIdentifier,
    ParsedBlockIdentifier,

    InBlock,

    // To figure out if what we're parsing is a block id or an attribute id
    ParsingIdentifier,

    ParsingAttrIdentifier,
    ParsedAttrIdentifier,

    ParsingEventListener,
    ParsedEventListener,

    InDirective,
    ParsingDirectiveIdentifier,
    ParsedDirectiveIdentifier,

    InDblQuoteUnescaped,
    InDblQuoteEscaped,
}

#[derive(Debug)]
struct Token {
    start: usize,
    end: usize,
    r#type: TokenType,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    // Keyword(?),
    BlockIdentifier(String),
    AttrIdentifier(String),
    StringExpr(String),
    LineComment(String),
    BlockComment(String),
    Colon,
    OpenCurly,
    CloseCurly,
    Expression,
    DirectiveOpen,
    DirectiveClose,
    DirectiveIdentifier(String),
    DirectiveColon,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlockIdentifier(str) => write!(f, "BlockIdentifier({str})"),
            AttrIdentifier(str) => write!(f, "AttrIdentifier({str})"),
            StringExpr(str) => write!(f, "StringExpr({str})"),
            LineComment(str) => write!(f, "LineComment({str})"),
            BlockComment(str) => write!(f, "BlockComment({str})"),
            Colon => write!(f, "Colon"),
            OpenCurly => write!(f, "OpenCurly"),
            CloseCurly => write!(f, "CloseCurly"),
            Expression => write!(f, "Expression"),
            DirectiveOpen => write!(f, "DirectiveOpen"),
            DirectiveClose => write!(f, "DirectiveClose"),
            DirectiveIdentifier(_) => write!(f, "DirectiveIdentifier"),
            DirectiveColon => write!(f, "DirectiveColon"),
        }
    }
}

struct Block {
    handle: String,
    r#type: BlockType,
    attributes: HashMap<String, String>,
    html: String,         // must be one of these three, maybe turn into enum
    text: String,         // must be one of these three, maybe turn into enum
    children: Vec<Block>, // must be one of these three, maybe turn into enum
}

enum BlockType {
    Page,
    Div,
    Heading,
    P,
    Html(&'static str), // Takes HTML tag as arg
}

fn main() {
    let mut tokenizer = Tokenizer {
        state_history: vec![Start],
        cursor: 0,
        line: 1,
        column: 0,
        buf: String::new(),
        buf_start: None,
        tokens: vec![],
    };

    let tokens = tokenizer.tokenize();
    dbg!(tokens);
}
