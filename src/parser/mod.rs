use std::fs::read_to_string;
use crate::parser::tokenizer::{Tokenizer, TokenizerResult};
use crate::parser::tokenizer::token::{Token, TokenType};

pub mod tokenizer;
pub mod tree_builder;

// todo: implement Action struct (for things like event handlers)
// todo: implement Expression struct (for directives like [if: $expr]
pub struct Parser {}

impl Parser {
    pub fn run(&mut self) {
        let mut tokenizer = Tokenizer::new();
        let mut builder = TreeBuilder::new();
        let string = read_to_string("test-stubs/basic.app").unwrap();

        for (cursor, char) in string.chars().enumerate() {
            let result = tokenizer.push_parse(char, cursor);
            match result {
                TokenizerResult::Parsing => {},
                TokenizerResult::Tokenized(tokens) => {
                    for token in tokens {
                        dbg!(&token);
                        let result = builder.push_token(token);
                        // match result {
                        //     BuilderResult::Ok() => {}
                        //     BuilderResult::Err(msg) => {}
                        // }
                    }
                }
                TokenizerResult::Err(err) => {
                    panic!("{}", err)
                }
            }
        }

        dbg!(&builder);

        // let tree = builder.build_tree();
        // Generator::new(target_dir).generate(tree);
    }
}

#[derive(Debug)]
struct TreeBuilder {
    tokens: Vec<Token>
}

impl TreeBuilder {
    fn new() -> TreeBuilder {
        TreeBuilder {
            tokens: vec![]
        }
    }

    fn push_token(&mut self, token: Token) {
        match self.tokens.last_mut() {
            // None means vec is empty, is start of file
            None => {
                match token.r#type() {
                    TokenType::UnresolvedIdentifier(_) => {
                        self.tokens.push(token);
                    },
                    TokenType::BlockIdentifier(_) |
                    TokenType::LineComment(_) |
                    TokenType::BlockComment(_) => self.tokens.push(token),
                    _ => panic!("Unexpected token"),
                }
            }
            Some(prev) => {
                match prev.r#type() {
                    TokenType::UnresolvedIdentifier(prev_id) => {
                        match token.r#type() {
                            TokenType::AttrColon | TokenType::DirectiveOpen => {
                                prev.set_type(TokenType::AttrIdentifier(prev_id.clone()));
                                self.tokens.push(token);
                            }
                            TokenType::BlockOpen => {
                                prev.set_type(TokenType::BlockIdentifier(prev_id.clone()));
                                self.tokens.push(token);
                            }

                            TokenType::UnresolvedIdentifier(_) => {}
                            TokenType::BlockIdentifier(_) => {}
                            TokenType::LineComment(_) => {}
                            TokenType::BlockComment(_) => {}
                            TokenType::BlockClose => {
                                // previous must be open curly, expression end, comma
                            }
                            TokenType::AttrIdentifier(_) => {}
                            TokenType::BlockOrAttrSeparator => {}
                            TokenType::ExprToken(_) => {}
                            TokenType::DirectiveClose => {}
                            TokenType::DirectiveIdentifier(_) => {}
                            TokenType::DirectiveColon => {}
                            TokenType::DirectiveValue(_) => {}
                            TokenType::EventListenerOpen => {}
                            TokenType::EventListenerClose => {}
                            TokenType::EventListenerIdentifier(_) => {}
                            TokenType::EventListenerColon => {}
                            TokenType::EventListenerHandler(_) => {}
                        }
                        // expect :, {, [
                    }
                    TokenType::BlockIdentifier(_) => {}
                    TokenType::LineComment(_) => {}
                    TokenType::BlockComment(_) => {}
                    TokenType::BlockOpen => {}
                    TokenType::BlockClose => {}
                    TokenType::AttrIdentifier(_) => {}
                    TokenType::AttrColon => {}
                    TokenType::BlockOrAttrSeparator => {}
                    TokenType::ExprToken(_) => {}
                    TokenType::DirectiveOpen => {}
                    TokenType::DirectiveClose => {}
                    TokenType::DirectiveIdentifier(_) => {}
                    TokenType::DirectiveColon => {}
                    TokenType::DirectiveValue(_) => {}
                    TokenType::EventListenerOpen => {}
                    TokenType::EventListenerClose => {}
                    TokenType::EventListenerIdentifier(_) => {}
                    TokenType::EventListenerColon => {}
                    TokenType::EventListenerHandler(_) => {}
                }
            }
        }
    }
}
