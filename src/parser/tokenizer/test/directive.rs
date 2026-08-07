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
#[should_panic]
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
fn t_0006_directives() {
    let string = file("directive/t_0006_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0007_directives() {
    let string = file("directive/t_0007_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0008_directives() {
    let string = file("directive/t_0008_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0009_directives() {
    let string = file("directive/t_0009_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0010_directives() {
    let string = file("directive/t_0010_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0011_directives() {
    let string = file("directive/t_0011_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0012_directives() {
    let string = file("directive/t_0012_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0013_directives() {
    let string = file("directive/t_0013_directives").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}