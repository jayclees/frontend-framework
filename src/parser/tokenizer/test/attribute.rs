use crate::parser::tokenizer::test::{assert_fails, file, match_tokens_to_source};
use crate::parser::tokenizer::Tokenizer;

#[test]
fn attribute_0001() {
    let string = file("attribute/attribute_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0002() {
    let string = file("attribute/attribute_0002").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0003() {
    let string = file("attribute/attribute_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0004() {
    let string = file("attribute/attribute_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0005() {
    let string = file("attribute/attribute_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0006() {
    let string = file("attribute/attribute_0006").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0007() {
    let string = file("attribute/attribute_0007").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0008() {
    let string = file("attribute/attribute_0008").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0009() {
    let string = file("attribute/attribute_0009").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0010() {
    let string = file("attribute/attribute_0010").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn attribute_0011_fail() {
    assert_fails(|| {
        let string = file("attribute/attribute_0011_fail").unwrap();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string.clone());
    }, r#"Unexpected token ":" at line: 1, column: 5. State: "ParsingIdentifier""#);
}
