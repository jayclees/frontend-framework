use crate::parser::tokenizer::TokenType;

#[derive(Debug)]
pub struct Token {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) r#type: TokenType,
}
