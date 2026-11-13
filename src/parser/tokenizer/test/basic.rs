use super::*;

#[test]
fn basic_0001() {
    let string = file("basic/basic_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn basic_0002() {
    let string = file("basic/basic_0002").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn basic_0003() {
    let string = file("basic/basic_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn basic_0004() {
    let string = file("basic/basic_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}
