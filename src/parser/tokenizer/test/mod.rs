mod basic;
mod token;

use super::Tokenizer;
use crate::parser::tokenizer::token::{Token, TokenType};
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;

fn base_path(path: &'static str) -> PathBuf {
    PathBuf::from("app").join(path)
}

fn file(path: &'static str) -> io::Result<String> {
    read_to_string(base_path(path))
}

#[test]
fn test_token_test_method() {
    let string = "Page {}";
    assert!(Token::new(0, TokenType::BlockIdentifier("Page".to_owned())).test(string));
    assert!(Token::new(5, TokenType::BlockOpen).test(string));
    assert!(Token::new(6, TokenType::BlockClose).test(string));
}

#[test]
fn test_token_test_method_wrong_token_type_1() {
    let string = "Page {}";
    assert!(!Token::new(5, TokenType::DirectiveOpen).test(string));
    assert!(!Token::new(6, TokenType::DirectiveClose).test(string));
}

#[test]
fn test_token_test_method_wrong_token_type_2() {
    let string = "Page {}";
    // Attempt to match BlockOpen with a } and vice versa
    assert!(!Token::new(5, TokenType::BlockClose).test(string));
    assert!(!Token::new(6, TokenType::BlockOpen).test(string));
}

#[test]
fn test_token_test_method_wrong_string_match() {
    let string = "Page {}";
    assert!(!Token::new(0, TokenType::BlockIdentifier("asdf".to_owned())).test(string));
}

#[test]
fn test_token_test_method_fails_on_shifted_string_match() {
    let string = "Page { body {} }";
    // Test match is good
    assert!(Token::new(7, TokenType::BlockIdentifier("body".to_owned())).test(string));

    // Test shifted matches
    assert!(!Token::new(6, TokenType::BlockIdentifier("body".to_owned())).test(string));
    assert!(!Token::new(8, TokenType::BlockIdentifier("body".to_owned())).test(string));
}
