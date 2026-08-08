use super::*;

#[test]
fn directive_0001() {
    let string = file("directive/directive_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
#[should_panic = "test"]
fn directive_0002_fail() {
    let string = file("directive/directive_0002_fail").unwrap();
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(string.clone());
}

#[test]
fn directive_0003() {
    let string = file("directive/directive_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn directive_0004() {
    let string = file("directive/directive_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn directive_0005() {
    let string = file("directive/directive_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        dbg!(token);
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn directive_0006() {
    let string = file("directive/directive_0006").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn directive_0007() {
    let string = file("directive/directive_0007").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn directive_0008() {
    let string = file("directive/directive_0008").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}


#[test]
fn directives_0001() {
    let string = file("directive/directives_0001").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
#[should_panic = "test"]
fn directives_0002_fail() {
    let string = file("directive/directives_0002_fail").unwrap();
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(string.clone());
}

#[test]
fn directives_0003() {
    let string = file("directive/directives_0003").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn directives_0004() {
    let string = file("directive/directives_0004").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn directives_0005() {
    let string = file("directive/directives_0005").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
#[should_panic = "test"]
fn directives_0006_fail() {
    let string = file("directive/directives_0006_fail").unwrap();
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(string.clone());
}

#[test]
#[should_panic = "test"]
fn directives_0007_fail() {
    let string = file("directive/directives_0007_fail").unwrap();
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(string.clone());
}

#[test]
#[should_panic = "test"]
fn directives_0008_fail() {
    let string = file("directive/directives_0008_fail").unwrap();
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(string.clone());
}
