use super::helper::{
    reset_buffer, current, pop_state, prev, push_state, push_state_keep_buf, push_token,
};
use super::{Token, TokenType, TokenizerContract};
use ExprState::*;

pub(super) struct ExpressionTokenizer {
    state_history: Vec<ExprState>,
    tokens: Vec<Token>,
    cursor: usize, // This is a reference to the position of the cursor for the source string/file
    sub_cursor: usize,
    buf: String,
    buf_start: Option<usize>,
}

impl ExpressionTokenizer {
    pub(super) fn new() -> ExpressionTokenizer {
        ExpressionTokenizer {
            state_history: vec![Start],
            tokens: vec![],
            cursor: 0,
            sub_cursor: 0,
            buf: String::new(),
            buf_start: None,
        }
    }

    fn reset_and_return(&mut self) -> ExprResult {
        self.reset_buffer();
        self.state_history = vec![Start];
        let mut taken = Vec::new();
        taken.append(&mut self.tokens);
        ExprResult::Parsed(taken)
    }

    pub(super) fn push_parse(&mut self, cursor: usize, char: char) -> ExprResult {
        self.cursor = cursor;

        // if cursor == 197 {
        //     crate::dd!("hello", char);
        // }
        // dbg!(cursor, char, self.state());

        match self.state() {
            Start => {
                dbg!(char);
                if char == '"' {
                    self.push_state(ParsingString, Some(char));
                } else if char.is_ascii_digit() {
                    self.push_state(ParsingNumber, Some(char));
                } else if char == ',' || char == '}' {
                    // todo:
                    // Potentially check if any tokens were parsed, if empty, or if
                    // buffer is not empty, return an error.
                    return self.reset_and_return();
                } else if !char.is_whitespace() {
                    return ExprResult::ErrUnexpected(
                        r#"Expected ", alphabetic, or number."#.to_owned(),
                    );
                }
            }
            ParsingString => {
                if char == '\\' {
                    self.push_state_keep_buf(ParsingStringEscaped, Some(char));
                } else if char == '"' {
                    self.pop_state();
                    self.push_state_keep_buf(ParsedString, Some(char));
                } else {
                    self.buf.push(char);
                }
            }
            ParsingStringEscaped => {
                self.pop_state();
                self.buf.push(char);
            }
            ParsedString => {
                self.push_token_pop_state(TokenType::ExprString(self.buf.clone()));
                if char == ',' {
                    return self.reset_and_return();
                }
            }
            ParsingNumber => {}
            ParsedNumber => {}
            ParsingFunctionCall => {}
            ParsedFunctionCall => {}
            ParsingOperator => {}
            ParsedOperator => {}
            ParsingParenthesesOpen => {}
            ParsedParenthesesOpen => {}
            ParsingParenthesesClosed => {}
            ParsedParenthesesClosed => {}
        }

        ExprResult::StillParsing
    }
}

impl TokenizerContract for ExpressionTokenizer {
    type TokenizerState = ExprState;

    fn state(&self) -> &ExprState {
        current(&self.state_history)
    }

    fn prev_state(&self) -> Option<&ExprState> {
        prev(&self.state_history)
    }

    fn push_state(&mut self, state: ExprState, starting_char: Option<char>) {
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

    fn reset_buffer(&mut self) {
        reset_buffer(&mut self.buf, &mut self.buf_start, self.cursor);
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum ExprState {
    Start,
    ParsingString,
    ParsingStringEscaped,
    ParsedString,
    ParsingNumber,
    ParsedNumber,
    ParsingFunctionCall,
    ParsedFunctionCall,
    ParsingOperator,
    ParsedOperator,
    ParsingParenthesesOpen,
    ParsedParenthesesOpen,
    ParsingParenthesesClosed,
    ParsedParenthesesClosed,
}

pub(super) enum ExprResult {
    StillParsing,
    Parsed(Vec<Token>),
    Err(String),
    ErrUnexpected(String),
}
