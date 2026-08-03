use super::token_type::Token;
use ExprState::*;

pub(super) struct ExpressionTokenizer<'a> {
    string: &'a str,
    state_history: Vec<ExprState>,
    file_cursor: usize, // This is a reference to the position of the cursor for the source string
    cursor: usize,
    buf: String,
    buf_start: Option<usize>,
}

impl<'a> ExpressionTokenizer<'a> {
    pub(super) fn new(cursor: usize, string: &'a str) -> ExpressionTokenizer<'a> {
        ExpressionTokenizer {
            string,
            state_history: vec![Start],
            file_cursor: cursor,
            cursor: 0,
            buf: String::new(),
            buf_start: None,
        }
    }

    pub(super) fn tokenize(&self) -> Vec<Token> {
        let tokens: Vec<Token> = vec![];
        let chars = self.string.chars();

        for char in chars.enumerate() {
            //
        }

        tokens
    }
}

pub(super) enum ExprState {
    Start,
    ParsingString,
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
