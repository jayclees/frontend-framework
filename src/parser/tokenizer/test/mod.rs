use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;
use crate::parser::tokenizer::token_type::{Token, TokenType};
use crate::parser::tokenizer::Tokenizer;

fn base_path(path: &'static str) -> PathBuf {
    PathBuf::from("app").join(path)
}

fn file(path: &'static str) -> io::Result<String> {
    read_to_string(base_path(path))
}

#[test]
fn t_0001_basic() {
    let string = file("basic/0001-basic").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string);
    assert_eq!(&vec![
        Token::new(0, 4, TokenType::BlockIdentifier("Page".to_owned())),
        Token::new(5, 6, TokenType::BlockOpen),
        Token::new(7, 8, TokenType::BlockClose),
    ], tokens);
}

#[test]
fn t_0002_basic() {
    let string = file("basic/0002-basic-no-space").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string);
    assert_eq!(&vec![
        Token::new(0, 4, TokenType::BlockIdentifier("Page".to_owned())),
        Token::new(4, 5, TokenType::BlockOpen),
        Token::new(5, 6, TokenType::BlockClose),
    ], tokens);
}
