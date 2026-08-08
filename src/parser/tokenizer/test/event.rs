use crate::parser::tokenizer::test::{assert_fails, file};
use crate::parser::tokenizer::Tokenizer;

#[test]
fn event_0001() {
    let string = file("event/event_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn event_0002_fail() {
    assert_fails(|| {
        let string = file("event/event_0002_fail").unwrap();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string.clone());
    }, r#"Unexpected token "@" at line: 1, column: 5. State: "ParsingIdentifier""#);
}

#[test]
fn event_0003() {
    let string = file("event/event_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn event_0004_fail() {
    assert_fails(|| {
        let string = file("event/event_0004_fail").unwrap();
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string.clone());
    }, r#"Unexpected token "{" at line: 1, column: 24. State: "ParsedDirectiveOrEventClose""#);
}

#[test]
fn event_0005() {
    let string = file("event/event_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn event_0006() {
    let string = file("event/event_0006").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn event_0007() {
    let string = file("event/event_0007").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn event_0008() {
    let string = file("event/event_0008").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}
