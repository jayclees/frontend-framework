use super::*;

#[test]
fn t_0001_directive() {
    let string = file("directive/t_0001_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0002_directive() {
    let string = file("directive/t_0002_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0003_directive() {
    let string = file("directive/t_0003_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0004_directive() {
    let string = file("directive/t_0004_directive").unwrap();
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(string.clone());
    for token in tokens {
        assert!(token.test(string.as_ref()))
    }
}

#[test]
fn t_0005_directive() {
    let string = file("directive/t_0005_directive").unwrap();
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
