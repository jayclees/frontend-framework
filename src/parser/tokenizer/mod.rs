pub mod directive;
mod pipeline;
mod state;
#[cfg(test)]
mod test;
pub(crate) mod token;

use crate::parser::tokenizer::directive::DirectiveEnum;
use crate::parser::tokenizer::pipeline::pipeline;
use state::State;
use std::cell::RefCell;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use token::{Token, TokenType};

pub struct Tokenizer<'token> {
    state_history: Vec<State>,
    char: char,
    cursor: usize,
    line: usize,
    column: usize,
    buf: String,
    buf_start: Option<usize>,
    tokens: Vec<Token>,
    active_directive: Option<DirectiveEnum<'token>>,
}

impl<'token> Tokenizer<'token> {
    pub fn new() -> Tokenizer<'token> {
        Tokenizer {
            state_history: vec![State::Start],
            char: 'a', // dummy char
            cursor: 0,
            line: 1,
            column: 0,
            buf: String::new(),
            buf_start: None,
            tokens: vec![],
            active_directive: None,
        }
    }

    pub fn tokenize(&mut self, string: String) -> &Vec<Token> {
        // todo:
        // Create method to attempt to recover from unexpected token and keep going? Recovery
        // method will depend on what state it is currently in and the state history.
        for (cursor, char) in string.chars().enumerate() {
            self.char = char;
            self.cursor = cursor;

            if char == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            match self.state() {
                State::Start => pipeline(
                    self,
                    vec![Pipe::ExpectIdentifierStart, Pipe::ExpectCommentStart],
                ),
                State::ResolvingCommentType => pipeline(self, vec![Pipe::ResolveCommentType]),
                State::ParsingLineComment => pipeline(self, vec![Pipe::ParseLineComment]),
                State::ParsingBlockComment => pipeline(self, vec![Pipe::ParseBlockComment]),
                State::ParsingIdentifier => pipeline(
                    self,
                    vec![
                        Pipe::ParseIdentifierChar,
                        Pipe::ResolveIdentifierType,
                        Pipe::ExpectBlockOpen,
                        Pipe::ExpectDirectiveOpen,
                        Pipe::ExpectEventListenerOpenStart,
                        Pipe::ExpectAttrColon,
                        Pipe::ExpectCommentStart,
                    ],
                ),
                State::ResolvingIdentifier => pipeline(
                    self,
                    vec![
                        Pipe::ResolveIdentifierType,
                        Pipe::ExpectBlockOpen,
                        Pipe::ExpectDirectiveOpen,
                        Pipe::ExpectEventListenerOpenStart,
                        Pipe::ExpectCommentStart,
                    ],
                ),
                State::ParsedBlockIdentifier => pipeline(
                    self,
                    vec![
                        Pipe::ExpectBlockOpen,
                        Pipe::ExpectDirectiveOpen,
                        Pipe::ExpectEventListenerOpenStart,
                        Pipe::ExpectCommentStart,
                    ],
                ),
                State::ParsedAttrIdentifier => {
                    pipeline(self, vec![Pipe::ExpectAttrColon, Pipe::ExpectCommentStart])
                }
                State::ParsingBlock => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectIdentifierStart,
                        Pipe::ExpectBlockClose,
                    ],
                ),
                State::ParsedAttrColon => pipeline(
                    self,
                    vec![
                        Pipe::ExpectOperandStart,
                        Pipe::ExpectNegativeOperator,
                        Pipe::ExpectParenthesisOpen,
                    ],
                ),
                State::ParsedExpressionTokens => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                    ],
                ),
                State::ParsingDirective => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectDirectiveIdentifierStart,
                    ],
                ),
                State::ParsingDirectiveIdentifier => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ParseDirectiveIdentifier,
                        Pipe::ExpectDirectiveColon,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ParsedDirectiveIdentifier => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectDirectiveColon,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ParsingDirectiveValue => pipeline(
                    self,
                    vec![
                        Pipe::SetStateForDirectiveValue,
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectNegativeOperator,
                        Pipe::ExpectOperandStart,
                        Pipe::ExpectParenthesisOpen,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::DirectiveExprStart => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectNegativeOperator,
                        Pipe::ExpectOperandStart,
                        Pipe::ExpectParenthesisOpen,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExpectDirectiveClose => pipeline(self, vec![Pipe::ExpectDirectiveClose]),
                State::ParsingForDirectiveValue => {
                    pipeline(self, vec![Pipe::ParseForDirectiveValue])
                }
                State::ParsingEventListenerOpen => {
                    pipeline(self, vec![Pipe::ParseEventListenerOpen])
                }
                State::ParsedEventListenerOpen => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectEventListenerIdentifierStart,
                    ],
                ),
                State::ParsingEventListenerIdentifier => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ParseEventListenerIdentifier,
                        Pipe::ExpectEventListenerIdentifierColon,
                    ],
                ),
                State::ParsedEventListenerIdentifier => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectEventListenerIdentifierColon,
                    ],
                ),
                State::ParsedEventListenerColon => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectEventListenerHandlerStart,
                    ],
                ),
                State::ParsingEventListenerHandler => pipeline(
                    self,
                    vec![
                        Pipe::ParseEventListenerHandler,
                        Pipe::ExpectEventListenerClose,
                    ],
                ),
                State::ExprStart => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectNegativeOperator,
                        Pipe::ExpectOperandStart,
                        Pipe::ExpectParenthesisOpen,
                    ],
                ),
                State::ExprParsingNumber => pipeline(
                    self,
                    vec![
                        Pipe::ParseNumber,
                        Pipe::ExpectSlash,
                        Pipe::ExpectOperatorStart,
                        Pipe::ExpectParenthesisClose,
                        Pipe::ExpectDot,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprParsedNumber => pipeline(
                    self,
                    vec![
                        Pipe::ExpectSlash,
                        Pipe::ExpectOperatorStart,
                        Pipe::ExpectParenthesisClose,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprParsingFractionalNumber => pipeline(
                    self,
                    vec![
                        Pipe::ParseFractionalNumber,
                        Pipe::ExpectSlash,
                        Pipe::ExpectOperatorStart,
                        Pipe::ExpectParenthesisClose,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprExpectStringStart => pipeline(self, vec![Pipe::ExpectStringStart]),
                State::ExprParsingString => pipeline(self, vec![Pipe::ParseString]),
                State::ExprParsedString => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectConcatenationOperator,
                        Pipe::ExpectPropAccessor,
                        Pipe::ExpectParenthesisClose,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprParsingVarOrKeyword => pipeline(
                    self,
                    vec![
                        Pipe::ParseVarOrKeyword,
                        Pipe::ExpectSlash,
                        Pipe::ExpectPropAccessor,
                        Pipe::ExpectBracketOpen,
                        Pipe::ExpectParenthesisClose,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprParsedVarOrKeyword => pipeline(
                    self,
                    vec![
                        Pipe::ExpectSlash,
                        Pipe::ExpectPropAccessor,
                        Pipe::ExpectBracketOpen,
                        Pipe::ExpectParenthesisClose,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprParsingOperator => pipeline(
                    self,
                    vec![
                        Pipe::ParseOperator,
                        Pipe::ExpectSlash,
                        Pipe::ExpectOperandStart,
                    ],
                ),
                State::ExprParsedOperator => pipeline(
                    self,
                    vec![
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectOperandStart,
                        Pipe::ExpectParenthesisOpen,
                    ],
                ),
                State::ExprParsedBracketOpen => pipeline(
                    self,
                    vec![
                        Pipe::UNIMPLEMENTED,
                        Pipe::ExpectCommentStart,
                        Pipe::ExpectOperandStart,
                    ],
                ),
                State::ExprParsedParenthesisClose => pipeline(
                    self,
                    vec![
                        Pipe::ExpectSlash,
                        Pipe::ExpectOperatorStart,
                        Pipe::ExpectParenthesisClose,
                        Pipe::ExpectPropAccessor,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprResolvingSlash => {
                    pipeline(self, vec![Pipe::ResolveSlash, Pipe::ExpectOperandStart])
                }
                State::ExprResolvingDot => pipeline(
                    self,
                    vec![
                        Pipe::ResolveDot,
                        Pipe::ExpectOperandStart,
                        Pipe::ExpectFractionalNumberStart,
                        Pipe::ExpectAttrSeparator,
                        Pipe::ExpectBlockClose,
                        Pipe::ExpectDirectiveClose,
                    ],
                ),
                State::ExprParsedNegativeOperator => pipeline(self, vec![Pipe::ExpectNumberStart]),
            }
        }

        if self.state_history.len() == 2 && self.state() == &State::ParsingLineComment {
            self.push_token(TokenType::LineComment(self.buf.clone()));
            self.pop_state_type(State::ParsingLineComment);
        }

        // All states should have been popped, and we should be back to start
        if self.state_history.len() != 1 && self.state() != &State::Start {
            self.err_eof();
        }

        // self.match_with_source();

        &self.tokens
    }

    fn dump_tokens(&self) {
        let mut output = String::new();
        for token in &self.tokens {
            output.push_str(token.r#type().symbol())
        }
        dbg!(output);
    }

    fn state(&self) -> &State {
        self.state_history
            .last()
            .expect("State history should not be empty.")
    }

    fn prev_state(&self) -> Option<&State> {
        self.state_history.get(self.state_history.len() - 2)
    }

    fn push_state(&mut self, state: State, starting_char: Option<char>) {
        self.state_history.push(state);

        if let Some(char) = starting_char {
            self.buf.push(char);
        }
    }

    fn pop_state(&mut self) {
        self.state_history.pop();
    }

    fn pop_state_type(&mut self, state: State) {
        let last = self.state_history.last().unwrap();
        if last != &state {
            dbg!(&self.tokens);
            dbg!(&self.state_history);
            panic!(
                r#"Expected to pop "{}", found "{}" instead.
Current State: "{}", Line: {}, Column: {}, Char: "{}""#,
                state,
                last,
                self.state(),
                self.line,
                self.column,
                self.char
            );
        }
        self.state_history.pop();
    }

    fn push_token(&mut self, r#type: TokenType) {
        self.tokens
            .push(Token::new(self.buf_start.expect("Should be set."), r#type));

        self.reset_buffer();
    }

    fn push_token_pop_state(&mut self, r#type: TokenType) {
        self.push_token(r#type);
        self.pop_state();
    }

    fn reset_buffer(&mut self) {
        self.buf.clear();
        self.buf_start.replace(self.cursor);
    }

    fn err_msg(&self, msg: String) {
        let msg = msg.trim_end_matches(".");
        panic!(
            r#"{}. At line: {}, column: {}. State: "{}""#,
            msg,
            self.line,
            self.column,
            self.state()
        );
    }

    fn err_unexpected(&self, msg: Option<String>) {
        // todo provide expected tokens based on current tokenizer state
        // dbg!(&self.tokens);
        // dbg!(&self.state_history);
        panic!(
            r#"Unexpected token "{}" at line: {}, column: {}. State: "{}"{}"#,
            self.char,
            self.line,
            self.column,
            self.state(),
            match msg {
                Some(msg) => {
                    format!(". {msg}")
                }
                None => "".to_owned(),
            }
        );
    }

    fn err_eof(&self) {
        eprintln!("Unexpected end of file. State history:");
        dbg!(&self.tokens);
        dbg!(&self.state_history);
        panic!("Unexpected end of file.");
    }

    fn panic_if(&self, condition: bool, msg: String) {
        if condition {
            panic!(
                r#"{}. At line: {}, column: {}. State: "{}""#,
                msg,
                self.line,
                self.column,
                self.state()
            );
        }
    }

    fn match_with_source(&self) {
        let source = read_to_string("test-stubs/welcome.app").unwrap();
        let mut source = RefCell::new(source.chars());
        let mut highlighted = String::new();
        let mut prev: Option<&Token> = None;
        for token in self.tokens.iter().clone() {
            let (ws_start, ws_stop) = if let Some(prev) = prev {
                (prev.end(), token.start())
            } else {
                (0, token.start())
            };
            let string = source.get_mut();
            let whitespace = string.take(ws_stop - ws_start).collect::<String>();
            let token_string = string.take(token.end() - token.start()).collect::<String>();

            let token_type = token.r#type().to_string();
            let display = if let Some(i) = token_type.find("(") {
                token_type[..i].to_owned()
            } else {
                token_type
            };
            highlighted.push_str(
                format!(
                    r#"{}<span class="{}">{}</span>"#,
                    whitespace,
                    display,
                    html_escape::encode_text(&token_string).as_ref()
                )
                .as_str(),
            );
            prev = Some(token);
        }

        // append remaining untokenized text
        highlighted.push_str(
            format!(
                r#"<span class="Untokenized">{}</span>"#,
                html_escape::encode_text(source.get_mut().as_str())
            )
            .as_str(),
        );

        let layout = read_to_string("generated/layout.html")
            .unwrap()
            .replace("{REPLACE}", highlighted.as_str());
        let bytes = layout.as_bytes();
        let written = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("generated/generated.html")
            .unwrap()
            .write(bytes);

        if let Ok(written) = written {
            if written == bytes.len() {
                println!("{written} bytes written to generated/generated.html");
            } else {
                eprintln!(
                    "Failed to successfully write to generated/generated.html. {}/{} bytes written.",
                    bytes.len(),
                    written
                );
            }
        } else {
            dbg!("Failed to write to generated/generated.html");
        }
    }
}

#[derive(Debug)]
enum PipelineResult {
    Handled,
    Continue,
    Err(String),
    ErrUnexpected,
}

pub enum Pipe {
    // Dummy pipe that will trigger "unimplemented" to mark unhandled tokenizer states
    #[allow(clippy::upper_case_acronyms)]
    UNIMPLEMENTED,

    ExpectIdentifierStart,
    ParseIdentifierChar,
    ResolveIdentifierType,

    ExpectCommentStart,
    ResolveCommentType,
    ParseLineComment,
    ParseBlockComment,

    ExpectBlockOpen,
    ExpectBlockClose,

    ExpectDirectiveOpen,
    ExpectDirectiveIdentifierStart,
    ParseDirectiveIdentifier,
    ExpectDirectiveColon,
    SetStateForDirectiveValue,
    ExpectDirectiveClose,

    ParseForDirectiveValue,

    ExpectEventListenerOpenStart,
    ParseEventListenerOpen,
    ExpectEventListenerIdentifierStart,
    ParseEventListenerIdentifier,
    ExpectEventListenerIdentifierColon,
    ExpectEventListenerHandlerStart,
    ParseEventListenerHandler,
    ExpectEventListenerClose,

    ExpectAttrColon,
    ExpectAttrSeparator,

    // Expressions
    ExpectOperandStart,
    ExpectNumberStart,
    ExpectParenthesisOpen,
    ExpectStringStart,
    ParseString,
    ParseNumber,
    ParseVarOrKeyword,
    ParseOperator,
    ExpectPropAccessor,
    ExpectBracketAccessorOpen,
    ExpectBracketAccessorClose,
    // ExpectFunctionArgsOpen,
    // ExpectFunctionArgsClose,
    ExpectNegativeOperator,
    ExpectOperatorStart,
    ExpectParenthesisClose,
    ExpectSlash,
    ExpectDot,
    ExpectBracketOpen,
    ResolveDot,
    ResolveSlash,
    ExpectFractionalNumberStart,
    ParseFractionalNumber,
    ExpectConcatenationOperator,
}
