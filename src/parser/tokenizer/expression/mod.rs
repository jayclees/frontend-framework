use super::helper::{
    current, pop_state, prev, push_state, push_state_keep_buf, push_token, reset_buffer,
};
use super::token_type::{Operator, OperatorMatchResult};
use super::{Token, TokenType, TokenizerContract};
use std::fmt::{Display, Formatter};
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

    fn get_tokens_and_reset(&mut self) -> ExprResult {
        self.reset_buffer();
        self.state_history = vec![Start];
        let mut taken = Vec::new();
        taken.append(&mut self.tokens);
        ExprResult::Parsed(taken)
    }

    fn err(&self, msg: String) -> ExprResult {
        ExprResult::Err(msg)
    }

    fn err_expected_expr_start(&self) -> ExprResult {
        ExprResult::ErrUnexpected(format!(
            r#"Expected ", alphabetic, or number. ExprState: "{}""#,
            self.state()
        ))
    }

    pub(super) fn push_parse(&mut self, cursor: usize, char: char) -> ExprResult {
        self.cursor = cursor;

        match self.state() {
            Start => {
                if char == '"' {
                    self.push_state(ParsingString, Some(char));
                } else if char.is_ascii_alphabetic() {
                    self.push_state(ParsingIdentifier, Some(char));
                } else if char.is_ascii_digit() || char == '-' {
                    self.push_state(ParsingNumber, Some(char));
                } else if char == ',' || char == '}' {
                    // todo:
                    // Potentially check if any tokens were parsed, if empty, or if
                    // buffer is not empty, return an error.
                    return self.get_tokens_and_reset();
                } else if !char.is_whitespace() {
                    return self.err_expected_expr_start();
                }
            }
            ParsedExprToken => {
                match self.prev_state().expect("Should have prev state.") {
                    ParsingString => {
                        self.push_token_pop_state(TokenType::ExprString(self.buf.clone()));
                        self.pop_state();
                    }
                    ParsingNumber => {
                        // self.push_token_pop_state(TokenType::ExprNumber(self.buf.clone()));
                        self.pop_state();
                        self.pop_state();
                    }
                    ParsingIdentifier => {
                        // decide whether it's a variable or function declaration
                    }
                    ParsingFunctionCall => {}
                    ParsingOperator => {}
                    ParsingParenthesesOpen => {}
                    ParsingParenthesesClosed => {}
                    _ => unimplemented!(),
                }

                if char.is_ascii_digit() {
                    self.push_state(ParsingNumber, Some(char));
                } else if char.is_ascii_alphabetic() {
                    self.push_state(ParsingIdentifier, Some(char));
                } else if Operator::is_operator_char(char) {
                    self.push_state(ParsingOperator, Some(char));
                } else if char == ',' || char == '}' {
                    return self.get_tokens_and_reset();
                } else if !char.is_whitespace() {
                    return self.err_expected_expr_start();
                }
            }
            ParsingString => {
                if char == '\\' {
                    self.push_state_keep_buf(ParsingStringEscaped, Some(char));
                } else if char == '"' {
                    self.push_state_keep_buf(ParsedExprToken, Some(char));
                } else {
                    self.buf.push(char);
                }
            }
            ParsingStringEscaped => {
                self.pop_state();
                self.buf.push(char);
            }
            ParsingNumber => {
                if char.is_ascii_digit() {
                    self.buf.push(char)
                } else if char == '.' {
                    if self.buf.is_empty() {
                        return self.err("Number cannot start with period.".to_owned());
                    }
                    self.buf.push(char)
                } else if char.is_whitespace() || char == ',' || char == '}' {
                    self.push_token(TokenType::ExprNumber(self.buf.clone()));
                    self.push_state(ParsedExprToken, Some(char));
                }
            }
            ParsedNumber => {}
            ParsingIdentifier => {}
            ParsingFunctionCall => {}
            ParsedFunctionCall => {}
            ParsingOperator => {
                if Operator::is_operator_char(char) {
                    self.buf.push(char);
                } else {
                    match Operator::get_match(&self.buf) {
                        OperatorMatchResult::Matched(operator) => {
                            self.push_token(TokenType::ExprOperator(operator))
                        }
                        OperatorMatchResult::Failed => {
                            return self.err(format!(r#"Failed operator match "{}""#, self.buf))
                        }
                    }
                    // todo handle current char
                }
            }
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
    ParsedExprToken,
    ParsingString,
    ParsingStringEscaped,
    ParsingNumber,
    ParsedNumber,
    ParsingIdentifier,
    ParsingFunctionCall,
    ParsedFunctionCall,
    ParsingOperator,
    ParsedOperator,
    ParsingParenthesesOpen,
    ParsedParenthesesOpen,
    ParsingParenthesesClosed,
    ParsedParenthesesClosed,
}

impl Display for ExprState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Start => write!(f, "Start"),
            ParsedExprToken => write!(f, "ParsedExprToken"),
            ParsingString => write!(f, "ParsingString"),
            ParsingStringEscaped => write!(f, "ParsingStringEscaped"),
            ParsingNumber => write!(f, "ParsingNumber"),
            ParsedNumber => write!(f, "ParsedNumber"),
            ParsingIdentifier => write!(f, "ParsingIdentifier"),
            ParsingFunctionCall => write!(f, "ParsingFunctionCall"),
            ParsedFunctionCall => write!(f, "ParsedFunctionCall"),
            ParsingOperator => write!(f, "ParsingOperator"),
            ParsedOperator => write!(f, "ParsedOperator"),
            ParsingParenthesesOpen => write!(f, "ParsingParenthesesOpen"),
            ParsedParenthesesOpen => write!(f, "ParsedParenthesesOpen"),
            ParsingParenthesesClosed => write!(f, "ParsingParenthesesClosed"),
            ParsedParenthesesClosed => write!(f, "ParsedParenthesesClosed"),
        }
    }
}

pub(super) enum ExprResult {
    StillParsing,
    Parsed(Vec<Token>),
    Err(String),
    ErrUnexpected(String),
}
