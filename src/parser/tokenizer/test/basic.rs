use super::*;

#[test]
fn basic_0001() {
    let string = file("basic/basic_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn basic_0002_fail() {
    assert_fails(|| {
        let string = file("basic/basic_0002_fail").unwrap();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string.clone());
    }, r#"Unexpected token "{" at line: 1, column: 5. State: "ParsingIdentifier""#);
}

#[test]
fn basic_0003() {
    let string = file("basic/basic_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    match_tokens_to_source(string.clone(), tokenizer.tokenize(string));
}

#[test]
fn basic_0004_fail() {
    assert_fails(|| {
        let string = file("basic/basic_0004_fail").unwrap();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string.clone());
    }, r#"Unexpected token "{" at line: 1, column: 5. State: "ParsingIdentifier""#);
}
