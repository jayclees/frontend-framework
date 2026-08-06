use super::*;

#[test]
fn t_0001_basic() {
    let string = file("basic/t_0001_basic").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0002_basic() {
    let string = file("basic/t_0002_basic").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0003_basic() {
    let string = file("basic/t_0003_basic").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0004_basic() {
    let string = file("basic/t_0004_basic").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0005_directive() {
    let string = file("basic/t_0005_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(&token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0006_directive() {
    let string = file("basic/t_0006_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(&token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0007_directive() {
    let string = file("basic/t_0007_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(&token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0008_directive() {
    let string = file("basic/t_0008_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(&token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0009_directive() {
    let string = file("basic/t_0009_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(&token);
        assert!(token.test(string.as_ref()))
    }
}
