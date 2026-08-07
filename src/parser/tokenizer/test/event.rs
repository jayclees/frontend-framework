use crate::parser::tokenizer::test::file;
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
#[should_panic]
fn event_0002_fail() {
    // Expect failure
    let string = file("event/event_0002_fail").unwrap();
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(string.clone());
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
fn event_0004() {
    let string = file("event/event_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}
