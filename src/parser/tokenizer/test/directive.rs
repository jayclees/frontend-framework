use super::*;

#[test]
fn directive_0001() {
    let string = file("directive/directive_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directive_0002() {
    let string = file("directive/directive_0002").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directive_0003() {
    let string = file("directive/directive_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directive_0004() {
    let string = file("directive/directive_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directive_0005() {
    let string = file("directive/directive_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directive_0006() {
    let string = file("directive/directive_0006").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directive_0007() {
    let string = file("directive/directive_0007").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directive_0008() {
    let string = file("directive/directive_0008").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0001() {
    let string = file("directive/directives_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0002() {
    let string = file("directive/directives_0002").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0003() {
    let string = file("directive/directives_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0004() {
    let string = file("directive/directives_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0005() {
    let string = file("directive/directives_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0006() {
    let string = file("directive/directives_0006").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0007() {
    let string = file("directive/directives_0007").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn directives_0008() {
    let string = file("directive/directives_0008").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}
