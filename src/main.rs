use crate::State::*;
use crate::TokenType::*;
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Tokenizer {
    state_history: Vec<State>,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn state(&self) -> &State {
        self.state_history.last().unwrap()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let string = read_to_string("app/root.app").unwrap();
        let block_id_reg = Regex::new(r#"\A[A-Z][a-zA-Z]\z"#).unwrap();
        let attr_id_reg = Regex::new(r#"\A[A-Z][a-zA-Z]\z"#).unwrap();
        let newline_reg = Regex::new(r#"\n"#).unwrap();
        let mut buf = String::new();
        // todo remove initial 0. this was set to satisfy compiler before implementation was complete
        let mut buf_start: usize = 0;

        // todo create method to attempt to recover from unexpected token and keep going?
        for (cursor, char) in string.chars().enumerate() {
            if char == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            match self.state() {
                Start => {
                    if char.is_alphabetic() {
                        buf.push(char);
                        self.state_history.push(ParsingBlockIdentifier)
                    } else if char.is_whitespace() {
                        // do nothing
                    } else if char == '/' {
                        let next = string.chars().nth(cursor + 1);

                        if next.is_none() {
                            self.panic("Unexpected end of file.");
                        }

                        if next.unwrap() == '/' {
                            self.state_history.push(InLineComment);
                            buf.push(char);
                            buf_start = cursor;
                        } else if next.unwrap() == '*' {
                            self.state_history.push(InBlockComment);
                            buf.push(char);
                            buf_start = cursor;
                        } else {
                            self.panic("Unexpected token.")
                        }
                    } else if char == '#' {
                        self.state_history.push(InLineComment);
                        buf.push(char);
                        buf_start = cursor;
                    } else {
                        self.panic("Unexpected token.");
                    }
                    // expect block identifier or comment
                }
                DefaultState => {
                    //
                }
                InLineComment => {
                    // ignore everything until new line
                    if char == '\n' {
                        let token = Token {
                            start: buf_start,
                            end: cursor,
                            r#type: LineComment(buf.clone()),
                        };
                        tokens.push(token);
                        buf.clear();
                        self.state_history.pop();
                    } else {
                        buf.push(char);
                    }
                }
                InBlockComment => {
                    //
                }
                ParsingBlockIdentifier => {
                    // match char against regex
                    // terminate on whitespace or curly, append token
                }
                ParsedBlockIdentifier => {
                    // todo expect optional control flow in square brackets after block id?
                    // expect whitespace or open curly
                    // enter into block scope on curly
                }
                InBlock => {
                    // expect attr id or block id
                }
                ParsingAttrIdentifier => {
                    // terminate if whitespace or colon
                    // if terminated on whitespace, assume an attribute like disabled in `<input disabled>`
                }
                ParsedAttrIdentifier => {
                    // was just terminated by whitespace or colon
                    // expect another attr id, or block id, or close curly
                }
                ParsingEventIdentifier => {
                    // match against event identifier regex
                    // terminate on colon
                }
                ParsedEventIdentifier => {
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

        tokens
    }

    fn panic(&self, msg: &'static str) {
        panic!("Line {}:{}: {}", self.line, self.column, msg)
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

    ParsingAttrIdentifier,
    ParsedAttrIdentifier,

    ParsingEventIdentifier,
    ParsedEventIdentifier,

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
}

impl TokenType {
    // fn expected(&self) -> Vec<TokenType> {
    //     match &self {
    //         BlockIdentifier(_) => vec![OpenCurly],
    //         AttrIdentifier(_) => vec![Colon],
    //         Colon => vec![Expression, StringExpr("".to_owned())],
    //         OpenCurly => vec![BlockIdentifier("".to_owned()), AttrIdentifier("".to_owned()), CloseCurly],
    //         CloseCurly => vec![BlockIdentifier("".to_owned())],
    //         StringExpr(_) | Expression => {
    //             vec![AttrIdentifier("".to_owned()), BlockIdentifier("".to_owned())]
    //         }
    //     }
    // }
    //
    // fn expects(&self, token: TokenType) -> bool {
    //     for expected in self.expected() {
    //         if expected.types_match(&token) {
    //             return true;
    //         }
    //     }
    //     false
    // }

    fn types_match(&self, token: &TokenType) -> bool {
        match self {
            BlockIdentifier(_) => {
                if let BlockIdentifier(_) = token {
                    true
                } else {
                    false
                }
            }
            AttrIdentifier(_) => {
                if let AttrIdentifier(_) = token {
                    true
                } else {
                    false
                }
            }
            StringExpr(_) => {
                if let StringExpr(_) = token {
                    true
                } else {
                    false
                }
            }
            LineComment(_) => {
                if let LineComment(_) = token {
                    true
                } else {
                    false
                }
            }
            BlockComment(_) => {
                if let BlockComment(_) = token {
                    true
                } else {
                    false
                }
            }
            Colon => token == &Colon,
            OpenCurly => token == &OpenCurly,
            CloseCurly => token == &CloseCurly,
            Expression => token == &Expression,
        }
    }
}

struct Block {
    handle: String,
    r#type: BlockType,
    attributes: HashMap<String, String>,
    html: String,              // must be one of these three, maybe turn into enum
    text: String,              // must be one of these three, maybe turn into enum
    children: Box<Vec<Block>>, // must be one of these three, maybe turn into enum
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
        line: 0,
        column: 0,
    };

    let tokens = tokenizer.tokenize();
    dbg!(tokens);
}
