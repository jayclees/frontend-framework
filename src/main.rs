use crate::State::*;
use crate::TokenType::*;
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::read_to_string;

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
        if starting_char.is_some() {
            self.buf.push(starting_char.unwrap());
            self.buf_start = Some(self.cursor);
        }
    }

    pub fn pop_state(&mut self) {
        self.buf.clear();
        self.state_history.pop();
    }

    pub fn push_token(&mut self, r#type: TokenType) {
        self.tokens.push(Token {
            start: self.buf_start.expect("buf_start should be set."),
            end: self.cursor,
            r#type,
        });
        self.pop_state();
    }

    fn err_unexpected(&self, char: char) {
        panic!(r#"Unexpected token "{char}"."#);
    }

    fn err_eof(&self) {
        panic!("Unexpected end of file.");
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        let string = read_to_string("app/my-page.app").unwrap();
        let mut skip: usize = 0;
        let _block_id_reg = Regex::new(r#"\A[A-Z][a-zA-Z]\z"#).unwrap();
        let _attr_id_reg = Regex::new(r#"\A[A-Z][a-zA-Z]\z"#).unwrap();
        let _newline_reg = Regex::new(r#"\n"#).unwrap();

        // todo create method to attempt to recover from unexpected token and keep going?
        for (cursor, char) in string.chars().enumerate() {
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
                        let next = string.chars().nth(cursor + 1);

                        if next.is_none() {
                            self.err_eof();
                        }

                        if next.unwrap() == '/' {
                            self.start_state(InLineComment, Some(char));
                        } else if next.unwrap() == '*' {
                            self.start_state(InBlockComment, Some(char));
                        } else {
                            self.err_unexpected(char);
                        }
                    } else if char == '#' {
                        self.start_state(InLineComment, Some(char));
                    } else {
                        self.err_unexpected(char);
                    }
                    // expect block identifier or comment
                }
                DefaultState => {
                    //
                }
                InLineComment => {
                    // ignore everything until new line
                    if char == '\n' {
                        self.push_token(LineComment(self.buf.clone()));
                    } else {
                        self.buf.push(char);
                    }
                }
                InBlockComment => {
                    //
                }
                ParsingBlockIdentifier => {
                    // allow numbers once first letter is identified as an ascii letter
                    if char.is_ascii_alphanumeric() {
                        self.buf.push(char);
                    } else if char.is_whitespace() || char == '[' || char == '{' {
                        self.push_token(BlockIdentifier(self.buf.clone()));
                        self.start_state(ParsedBlockIdentifier, Some(char));
                    } else {
                        self.err_unexpected(char);
                    }
                    // match char against regex
                    // terminate on whitespace or curly, append token
                }
                ParsedBlockIdentifier => {
                    if !char.is_whitespace() && char == '{' {
                        self.push_token(OpenCurly);
                        self.start_state(InBlock, None);
                    } else if !char.is_whitespace() && char == '[' {
                        self.start_state(ParsingDirective, None);
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
                    // todo expect optional control flow in square brackets after block id?
                }
                InBlock => {
                    // expect attr id or block id or event id
                    if char.is_ascii_alphabetic() {
                        self.start_state(ParsingAttrIdentifier, Some(char));
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
                ParsingDirective => {
                    if char.is_ascii_alphabetic() {
                        
                    }
                    // close on close bracket
                }
                ParsedDirective => {
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

    ParsingDirective,
    ParsedDirective,

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
