mod basic;
mod token;
mod directive;
mod event;

use super::Tokenizer;
use crate::parser::tokenizer::token::{Token, TokenType};
use std::fs::read_to_string;
use std::io;
use std::panic::UnwindSafe;
use std::path::PathBuf;
use crate::parser::tokenizer::token::TokenType::{BlockClose, BlockOpen, DirectiveClose, DirectiveColon, DirectiveIdentifier, DirectiveOpen, DirectiveValue, EventListenerClose, EventListenerColon, EventListenerHandler, EventListenerIdentifier, EventListenerOpen};

fn base_path(path: &'static str) -> PathBuf {
    PathBuf::from("app").join(path)
}

fn file(path: &'static str) -> io::Result<String> {
    read_to_string(base_path(path))
}

fn assert_fails<R, F: FnOnce() -> R + UnwindSafe>(f: F, expected_msg: &'static str) {
    let r = std::panic::catch_unwind(f);

    assert!(r.is_err(), "panic! was expected, but no panic! occurred.");
    let err = r.err().unwrap();
    let msg = err.as_ref().downcast_ref::<String>().unwrap();
    assert_eq!(expected_msg, *msg);
}

// todo
fn match_tokens_to_source(source: String, tokens: &Vec<Token>) {
    for token in tokens {
        dbg!(token, &source[token.start()..token.end()]);
        assert!(token.test(source.as_ref()))
    }
}

#[test]
fn token_test_method() {
    let string = "Page {}";
    assert!(Token::new(0, TokenType::BlockIdentifier("Page".to_owned())).test(string));
    assert!(Token::new(5, TokenType::BlockOpen).test(string));
    assert!(Token::new(6, TokenType::BlockClose).test(string));
}

#[test]
fn token_test_method_wrong_token_type_1() {
    let string = "Page {}";
    assert!(!Token::new(5, TokenType::DirectiveOpen).test(string));
    assert!(!Token::new(6, TokenType::DirectiveClose).test(string));
}

#[test]
fn token_test_method_wrong_token_type_2() {
    let string = "Page {}";
    // Attempt to match BlockOpen with a } and vice versa
    assert!(!Token::new(5, TokenType::BlockClose).test(string));
    assert!(!Token::new(6, TokenType::BlockOpen).test(string));
}

#[test]
fn token_test_method_wrong_string_match() {
    let string = "Page {}";
    assert!(!Token::new(0, TokenType::BlockIdentifier("asdf".to_owned())).test(string));
}

#[test]
fn token_test_method_fails_on_shifted_string_match() {
    let string = "Page { body {} }";
    // Test match is good
    assert!(Token::new(7, TokenType::BlockIdentifier("body".to_owned())).test(string));

    // Test shifted matches
    assert!(!Token::new(6, TokenType::BlockIdentifier("body".to_owned())).test(string));
    assert!(!Token::new(8, TokenType::BlockIdentifier("body".to_owned())).test(string));
}

#[test]
fn test_match_tokens_to_source() {
    let source = r#"Page [hello: "world"] @[click: do_thing()] {}"#.to_owned();
    let tokens = vec![
        Token::new(0, DirectiveIdentifier("Page".to_owned())),
        Token::new(5, DirectiveOpen),
        Token::new(6, DirectiveIdentifier("hello".to_owned())),
        Token::new(11, DirectiveColon),
        Token::new(12, DirectiveValue(r#" "world""#.to_owned())),
        Token::new(20, DirectiveClose),
        Token::new(22, EventListenerOpen),
        Token::new(24, EventListenerIdentifier("click".to_owned())),
        Token::new(29, EventListenerColon),
        Token::new(30, EventListenerHandler(" do_thing()".to_owned())),
        Token::new(41, EventListenerClose),
        Token::new(43, BlockOpen),
        Token::new(44, BlockClose),
    ];
    match_tokens_to_source(source, &tokens);
}

#[test]
fn test_assert_fn_passes_when_panic_occurs() {
    assert_fails(|| {
        let string = "Page{}".to_owned();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string);
    }, r#"Unexpected token "{" at line: 1, column: 5. State: "ParsingIdentifier""#);
}

#[test]
#[should_panic = "panic! was expected, but no panic! occurred."]
fn test_assert_fails_fn_fails_when_no_panic_occurs() {
    assert_fails(|| {
        // This is properly formatted string that will throw no errors
        let string = "Page {}".to_owned();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string);
    }, "");
}
