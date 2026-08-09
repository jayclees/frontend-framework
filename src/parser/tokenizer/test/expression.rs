use crate::parser::tokenizer::test::{file, match_tokens_to_source};
use crate::parser::tokenizer::Tokenizer;

#[test]
fn string_0001() {
    let string = file("expression/string_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn string_0002() {
    let string = file("expression/string_0002").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn string_0003() {
    let string = file("expression/string_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn string_0004() {
    let string = file("expression/string_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn string_0005() {
    let string = file("expression/string_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn string_0006() {
    let string = file("expression/string_0006").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn number_0001() {
    let string = file("expression/number_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn number_0002() {
    let string = file("expression/number_0002").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn number_0003() {
    let string = file("expression/number_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn number_0004() {
    let string = file("expression/number_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn number_0005() {
    let string = file("expression/number_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn number_0006() {
    let string = file("expression/number_0006").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}
