use super::helper::{
    current, pop_state, pop_state_type, prev, push_state, push_state_keep_buf, push_token,
    reset_buffer,
};
use super::token::{Expr, Operator, OperatorMatchResult};
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
                } else if char.is_ascii_digit() {
                    self.push_state(ParsingNumber, Some(char));
                } else if char == '-' {
                    self.reset_buffer();
                    self.buf.push(char);
                    self.push_token(TokenType::ExprToken(Expr::Operator(Operator::Minus)));
                } else if char == ',' || char == '}' {
                    if self.tokens.is_empty() {
                        return self.err("Expected expression.".to_owned());
                    }
                    if !self.buf.is_empty() {
                        eprintln!(r#"Buffer: "{}""#, self.buf);
                        panic!(
                            "Internal error: Buffer not empty when attempting to exit expression tokenizer."
                        )
                    }

                    return self.get_tokens_and_reset();
                } else if !char.is_whitespace() {
                    return self.err_expected_expr_start();
                }
            }
            ParsedExprToken => {
                match self.prev_state().expect("Should have prev state.") {
                    ParsingString => {
                        self.push_token_pop_state(TokenType::ExprToken(Expr::String(
                            self.buf.clone(),
                        )));
                        self.pop_state();
                    }
                    ParsingNumber => {
                        // self.push_token_pop_state(TokenType::ExprNumber(self.buf.clone()));
                        self.pop_state();
                        self.pop_state();
                    }
                    ParsingIdentifier => {
                        todo!("ParsingIdentifier");
                        // decide whether it's a variable or function declaration
                    }
                    ParsingFunctionCall => todo!("ParsingFunctionCall"),
                    ParsingOperator => todo!("ParsingOperator"),
                    ParsingParenthesesOpen => todo!("ParsingParenthesesOpen"),
                    ParsingParenthesesClosed => todo!("ParsingParenthesesClosed"),
                    _ => unimplemented!(),
                }

                if !self.buf.is_empty() {
                    if self.buf.len() > 1 {
                        panic!("Buffer should not be greater than 1 at this point.")
                    }

                    let last = self.buf.chars().last().unwrap();
                } else if char.is_ascii_digit() {
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
                    // period
                    if self.buf.contains('.') {
                        return self.err("Number can only have one dot.".to_owned());
                    }
                    self.buf.push(char);
                } else if char == ',' || char == '}' {
                    self.push_token(TokenType::ExprToken(Expr::Number(self.buf.clone())));
                    return self.get_tokens_and_reset();
                } else if char.is_whitespace() || char == ',' || char == '}' {
                    self.push_token(TokenType::ExprToken(Expr::Number(self.buf.clone())));
                    self.push_state(ParsedExprToken, Some(char));
                }
            }
            ParsedNumber => todo!("ParsedNumber"),
            ParsingIdentifier => todo!("ParsingIdentifier"),
            ParsingFunctionCall => todo!("ParsingFunctionCall"),
            ParsedFunctionCall => todo!("ParsedFunctionCall"),
            ParsingOperator => {
                if Operator::is_operator_char(char) {
                    self.buf.push(char);
                } else {
                    match Operator::get_match(&self.buf) {
                        OperatorMatchResult::Matched(operator) => {
                            self.push_token(TokenType::ExprToken(Expr::Operator(operator)));
                            // expect next token to be string, number, variable, function call
                            if !char.is_whitespace() {
                                self.push_state(ParsedExprToken, Some(char));
                            } else {
                                self.push_state(ParsedExprToken, None);
                            }
                        }
                        OperatorMatchResult::Failed => {
                            return self.err(format!(r#"Failed operator match "{}""#, self.buf));
                        }
                    }
                    // todo handle current char
                }
            }
            ParsedOperator => todo!("ParsedOperator"),
            ParsingParenthesesOpen => todo!("ParsingParenthesesOpen"),
            ParsedParenthesesOpen => todo!("ParsedParenthesesOpen"),
            ParsingParenthesesClosed => todo!("ParsingParenthesesClosed"),
            ParsedParenthesesClosed => todo!("ParsedParenthesesClosed"),
            ResolvingMinusSignType => {
                // maybe keep pushing to buffer until we know for sure if it's a minus operator
                // or negative sign, then break up the buffer into known tokens
                // potential cases:
                // -1
                // var--
                // --var
                // prev_token - 1
                // prev_token - -1

                // todo figure out when buffer is ready to be parsed

                // if prev token == number && buf == "-", expect '-'
                if self.buf.as_str() == "-" && char == '-' {
                    //
                }
                // if previous token is a number, expect either number
                else if let Some(token) = self.tokens.last()
                    && let TokenType::ExprToken(expr) = token.r#type()
                    && let Expr::Number(num) = expr
                {
                    //
                }
            }
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

    fn pop_state_type(&mut self, state: Self::TokenizerState) {
        pop_state_type(&mut self.state_history, state);
    }

    fn push_token(&mut self, r#type: TokenType) {
        push_token(
            &mut self.tokens,
            self.buf_start.expect("Should be set."),
            r#type,
        );
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
    ResolvingMinusSignType,
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
            ResolvingMinusSignType => write!(f, "ResolvingMinusSignType"),
        }
    }
}

pub(super) enum ExprResult {
    StillParsing,
    Parsed(Vec<Token>),
    Err(String),
    ErrUnexpected(String),
}
