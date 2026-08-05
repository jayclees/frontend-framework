use crate::parser::tokenizer::token_type::{Token, TokenType};

pub trait TokenizerContract {
    type TokenizerState;
    fn state(&self) -> &Self::TokenizerState;
    fn prev_state(&self) -> Option<&Self::TokenizerState>;
    fn push_state(&mut self, state: Self::TokenizerState, starting_char: Option<char>);
    fn push_state_include_ws(&mut self, state: Self::TokenizerState, starting_char: Option<char>);
    fn push_state_keep_buf(&mut self, state: Self::TokenizerState, starting_char: Option<char>);
    fn pop_state(&mut self);
    fn pop_state_type(&mut self, state: Self::TokenizerState);
    fn push_token(&mut self, r#type: TokenType);
    fn squeeze_token(&mut self, r#type: TokenType);
    fn push_token_pop_state(&mut self, r#type: TokenType);
    fn buf_last(&self) -> Option<char>;
    fn reset_buffer(&mut self);
}

pub(super) fn current<T>(state_history: &[T]) -> &T {
    state_history
        .last()
        .expect("State history should not be empty.")
}

pub(super) fn prev<T>(state_history: &[T]) -> Option<&T> {
    state_history.get(state_history.len() - 2)
}

pub(super) fn push_state<T>(
    state_history: &mut Vec<T>,
    state: T,
    buf: &mut String,
    buf_start: &mut Option<usize>,
    starting_char: Option<char>,
    cursor: usize,
) {
    state_history.push(state);
    reset_buffer(buf, buf_start, cursor);

    if let Some(char) = starting_char && !char.is_whitespace() {
        buf.push(char);
    }
}

pub(super) fn push_state_include_ws<T>(
    state_history: &mut Vec<T>,
    state: T,
    buf: &mut String,
    buf_start: &mut Option<usize>,
    starting_char: Option<char>,
    cursor: usize,
) {
    state_history.push(state);
    reset_buffer(buf, buf_start, cursor);

    if let Some(char) = starting_char {
        buf.push(char);
    }
}

pub(super) fn push_state_keep_buf<T>(
    state_history: &mut Vec<T>,
    state: T,
    buf: &mut String,
    starting_char: Option<char>,
) {
    state_history.push(state);

    if let Some(char) = starting_char {
        buf.push(char);
    }
}

pub(super) fn pop_state<T>(state_history: &mut Vec<T>) {
    state_history.pop();
}

pub(super) fn pop_state_type<T: PartialEq<T>>(state_history: &mut Vec<T>, state: T) {
    let last = state_history.last().expect("Should always have at least 1 state_history (State::Start)");
    if last != &state {
        panic!("State types do not match.");
    }
    state_history.pop();
}

pub(super) fn push_token(
    tokens: &mut Vec<Token>,
    r#type: TokenType,
    buf_start: usize,
    buf_end: usize,
) {
    let start = buf_start;
    let end = buf_end;

    if start == end {
        dbg!(r#type, start, end);
        panic!("Token should not be zero length.");
    }

    tokens.push(Token { start, end, r#type });
}

pub(super) fn reset_buffer(buf: &mut String, buf_start: &mut Option<usize>, cursor: usize) {
    buf.clear();
    buf_start.replace(cursor);
}
