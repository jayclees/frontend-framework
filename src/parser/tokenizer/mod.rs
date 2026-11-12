mod expression;
mod helper;
mod state;
#[cfg(test)]
mod test;
pub(crate) mod token;

use crate::parser::tokenizer::expression::ExprResult;
use expression::ExpressionTokenizer;
use helper::{
    TokenizerContract, current, pop_state, pop_state_type, prev, push_state, push_state_keep_buf,
    push_token, reset_buffer,
};
use state::State;
use std::cell::RefCell;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use token::{Token, TokenType};

#[derive(Debug)]
pub struct Tokenizer {
    state_history: Vec<State>,
    char: char,
    cursor: usize,
    line: usize,
    column: usize,
    buf: String,
    buf_start: Option<usize>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            state_history: vec![State::Start],
            char: 'a', // dummy char
            cursor: 0,
            line: 1,
            column: 0,
            buf: String::new(),
            buf_start: None,
            tokens: vec![],
        }
    }
}

impl Tokenizer {
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
        let source = read_to_string("app/my-page.app").unwrap();
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

    fn expect_identifier_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphabetic() {
            self.reset_buffer();
            self.buf.push(char);
            return PipelineResult::HandledPushState(State::ParsingIdentifier);
        }

        PipelineResult::Continue
    }

    fn parse_identifier_char(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphanumeric() {
            self.buf.push(char);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn resolve_identifier_type(&mut self) -> PipelineResult {
        let char = self.char;

        if matches!(char, '[' | '@' | '{') {
            self.push_token_pop_state(TokenType::BlockIdentifier(self.buf.clone()));
        }

        if char == ':' {
            self.push_token_pop_state(TokenType::AttrIdentifier(self.buf.clone()));
        }

        PipelineResult::Continue
    }

    fn expect_comment_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '/' {
            self.reset_buffer();
            self.push_state(State::ResolvingCommentType, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn resolve_comment_type(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '/' {
            self.pop_state();
            self.push_state_keep_buf(State::ParsingLineComment, Some(char));
            return PipelineResult::Handled;
        }

        if char == '*' {
            self.pop_state();
            self.push_state_keep_buf(State::ParsingBlockComment, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::ErrUnexpected
    }

    fn parse_line_comment(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '\n' {
            self.buf.push(char);
            self.push_token_pop_state(TokenType::LineComment(self.buf.clone()));
        } else {
            self.buf.push(char);
        }

        PipelineResult::Handled
    }

    fn parse_block_comment(&mut self) -> PipelineResult {
        let char = self.char;

        if let Some(last) = self.buf.chars().last()
            && last == '*'
            && char == '/'
        {
            self.buf.push(char);
            self.push_token(TokenType::BlockComment(self.buf.clone()));
            self.pop_state_type(State::ParsingBlockComment);
        } else {
            self.buf.push(char);
        }

        PipelineResult::Handled
    }

    fn expect_block_open(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '{' {
            self.reset_buffer();
            self.push_token(TokenType::BlockOpen);
            self.push_state(State::ParsingBlock, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_block_close(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '}' {
            self.reset_buffer();
            self.push_token(TokenType::BlockClose);
            self.pop_state_type(State::ParsingBlock);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_directive_open(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '[' {
            self.reset_buffer();
            self.push_token(TokenType::DirectiveOpen);
            self.push_state(State::ParsingDirective, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_directive_identifier_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphabetic() {
            self.reset_buffer();
            self.push_state();
        }
    }

    fn parse_directive_identifier(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_directive_colon(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_directive_value(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn parse_directive_value(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_directive_close(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_event_listener_open_start(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn parse_event_listener_open(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_event_listener_identifier_start(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_event_listener_identifier_colon(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn parse_event_listener_identifier(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_event_listener_handler_start(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn parse_event_listener_handler(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_event_listener_close(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_attr_colon(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_expression_start(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn expect_attr_separator(&mut self) -> PipelineResult {
        let char = self.char;

        //
    }

    fn pipeline(&mut self, pipes: Vec<Pipe>) {
        for pipe in pipes {
            let result = match pipe {
                Pipe::ExpectIdentifierStart => self.expect_identifier_start(),
                Pipe::ParseIdentifierChar => self.parse_identifier_char(),
                Pipe::ResolveIdentifierType => self.resolve_identifier_type(),
                Pipe::ExpectCommentStart => self.expect_comment_start(),
                Pipe::ResolveCommentType => self.resolve_comment_type(),
                Pipe::ParseLineComment => self.parse_line_comment(),
                Pipe::ParseBlockComment => self.parse_block_comment(),
                Pipe::ExpectBlockOpen => self.expect_block_open(),
                Pipe::ExpectBlockClose => self.expect_block_close(),
                Pipe::ExpectDirectiveOpen => self.expect_directive_open(),
                Pipe::ExpectDirectiveIdentifierStart => self.expect_directive_identifier_start(),
                Pipe::ParseDirectiveIdentifier => self.parse_directive_identifier(),
                Pipe::ExpectDirectiveColon => self.expect_directive_colon(),
                Pipe::ExpectDirectiveValue => self.expect_directive_value(),
                Pipe::ParseDirectiveValue => self.parse_directive_value(),
                Pipe::ExpectDirectiveClose => self.expect_directive_close(),
                Pipe::ExpectEventListenerOpenStart => self.expect_event_listener_open_start(),
                Pipe::ParseEventListenerOpen => self.parse_event_listener_open(),
                Pipe::ExpectEventListenerIdentifierStart => {
                    self.expect_event_listener_identifier_start()
                }
                Pipe::ExpectEventListenerIdentifierColon => {
                    self.expect_event_listener_identifier_colon()
                }
                Pipe::ParseEventListenerIdentifier => self.parse_event_listener_identifier(),
                Pipe::ExpectEventListenerHandlerStart => self.expect_event_listener_handler_start(),
                Pipe::ParseEventListenerHandler => self.parse_event_listener_handler(),
                Pipe::ExpectEventListenerClose => self.expect_event_listener_close(),
                Pipe::ExpectAttrColon => self.expect_attr_colon(),
                Pipe::ExpectExpressionStart => self.expect_expression_start(),
                Pipe::ExpectAttrSeparator => self.expect_attr_separator(),
            };
            match result {
                PipelineResult::Handled => return,
                PipelineResult::Continue | PipelineResult::Void => { /* Do nothing */ }
                PipelineResult::ReplaceState(next) => {
                    self.pop_state();
                    self.push_state(next, None);
                    return;
                }
                PipelineResult::HandledPushState(next) => return self.push_state(next, None),
                PipelineResult::PushStateKeepBuf(next, char) => {
                    return self.push_state_keep_buf(next, char);
                }
                PipelineResult::PopState => {}
                PipelineResult::PopStateType(_) => {}
                PipelineResult::Err => {}
                PipelineResult::ErrUnexpected => self.err_unexpected(None),
            };
        }
    }

    pub fn tokenize(&mut self, string: String) -> &Vec<Token> {
        let mut expr_tokenizer = ExpressionTokenizer::new();

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
                State::Start => {
                    self.pipeline(vec![Pipe::ExpectIdentifierStart, Pipe::ExpectCommentStart])
                }
                State::ResolvingCommentType => self.pipeline(vec![Pipe::ResolveCommentType]),
                State::ParsingLineComment => self.pipeline(vec![Pipe::ParseLineComment]),
                State::ParsingBlockComment => self.pipeline(vec![Pipe::ParseBlockComment]),
                State::ParsingIdentifier => self.pipeline(vec![
                    Pipe::ParseIdentifierChar,
                    Pipe::ResolveIdentifierType,
                    Pipe::ExpectBlockOpen,
                    Pipe::ExpectDirectiveOpen,
                    Pipe::ExpectEventListenerOpenStart,
                    Pipe::ExpectCommentStart,
                ]),
                State::ResolvingIdentifier => self.pipeline(vec![
                    Pipe::ResolveIdentifierType,
                    Pipe::ExpectBlockOpen,
                    Pipe::ExpectDirectiveOpen,
                    Pipe::ExpectEventListenerOpenStart,
                    Pipe::ExpectCommentStart,
                ]),
                State::ParsedBlockIdentifier => self.pipeline(vec![
                    Pipe::ExpectBlockOpen,
                    Pipe::ExpectDirectiveOpen,
                    Pipe::ExpectEventListenerOpenStart,
                    Pipe::ExpectCommentStart,
                ]),
                State::ParsedAttrIdentifier => {
                    self.pipeline(vec![Pipe::ExpectAttrColon, Pipe::ExpectCommentStart])
                }
                State::ParsingBlock => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectIdentifierStart,
                    Pipe::ExpectBlockClose,
                ]),
                State::ParsedAttrColon => {
                    self.pipeline(vec![Pipe::ExpectCommentStart, Pipe::ExpectExpressionStart])
                }
                State::GatheringExpressionTokens => {
                    match expr_tokenizer.push_parse(cursor, char) {
                        ExprResult::StillParsing => {
                            // continue
                        }
                        ExprResult::Parsed(mut tokens) => {
                            self.tokens.append(&mut tokens);
                            self.pop_state();
                            if char == '}' {
                                self.reset_buffer();
                                self.push_token(TokenType::BlockClose);
                                self.pop_state_type(State::ParsingBlock);
                            } else if char == ',' {
                                self.reset_buffer();
                                self.push_token(TokenType::AttrSeparator);
                            } else {
                                self.push_state(State::ParsedExpressionTokens, Some(char));
                            }
                        }
                        ExprResult::Err(msg) => {
                            self.err_msg(msg);
                        }
                        ExprResult::ErrExpectedExprStart(msg) => {
                            self.err_unexpected(Some(msg));
                        }
                        ExprResult::ErrUnexpected(expr_state) => {
                            self.err_unexpected(Some(format!(
                                "Expr tokenizer state: {expr_state}"
                            )));
                        }
                        ExprResult::ParsedBlockIdentifier(_) => {}
                    }
                }
                State::ParsedExpressionTokens => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                ]),
                State::ParsingDirective => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectDirectiveIdentifierStart,
                ]),
                State::ParsingDirectiveIdentifier => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ParseDirectiveIdentifier,
                    Pipe::ExpectDirectiveColon,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ParsedDirectiveIdentifier => {
                    self.pipeline(vec![Pipe::ExpectCommentStart, Pipe::ExpectDirectiveColon])
                }
                State::ParsedDirectiveColon => {
                    self.pipeline(vec![Pipe::ExpectCommentStart, Pipe::ExpectDirectiveValue])
                }
                State::ParsingDirectiveValue => self.pipeline(vec![Pipe::ParseDirectiveValue]),
                State::ParsingEventListenerOpen => {
                    self.pipeline(vec![Pipe::ParseEventListenerOpen])
                }
                State::ParsedEventListenerOpen => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectEventListenerIdentifierStart,
                ]),
                State::ParsingEventListenerIdentifier => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ParseEventListenerIdentifier,
                    Pipe::ExpectEventListenerIdentifierColon,
                ]),
                State::ParsedEventListenerColon => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectEventListenerHandlerStart,
                ]),
                State::ParsingEventListenerHandler => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ParseEventListenerHandler,
                    Pipe::ExpectEventListenerClose,
                ]),
            }
        }

        // All states should have been popped, and we should be back to start
        if self.state_history.len() != 1 && self.state() != &State::Start {
            self.err_eof();
        }

        // self.match_with_source();

        &self.tokens
    }
}

impl TokenizerContract for Tokenizer {
    type TokenizerState = State;

    fn state(&self) -> &State {
        current(&self.state_history)
    }

    fn prev_state(&self) -> Option<&State> {
        prev(&self.state_history)
    }

    fn push_state(&mut self, state: State, starting_char: Option<char>) {
        push_state(
            &mut self.state_history,
            state,
            &mut self.buf,
            starting_char,
        );
    }

    fn push_state_keep_buf(&mut self, state: Self::TokenizerState, starting_char: Option<char>) {
        push_state_keep_buf(&mut self.state_history, state, &mut self.buf, starting_char);
    }

    fn pop_state(&mut self) {
        pop_state(&mut self.state_history);
    }

    fn pop_state_type(&mut self, state: Self::TokenizerState) {
        pop_state_type(&mut self.state_history, state);
    }

    fn push_token(&mut self, r#type: TokenType) {
        let buf_start = self.buf_start.expect("Should be set.");
        push_token(&mut self.tokens, buf_start, r#type);
        self.reset_buffer();
    }

    fn push_token_pop_state(&mut self, r#type: TokenType) {
        self.push_token(r#type);
        self.pop_state();
    }

    fn buf_last(&self) -> Option<char> {
        self.buf.chars().last()
    }

    fn reset_buffer(&mut self) {
        reset_buffer(&mut self.buf, &mut self.buf_start, self.cursor);
    }
}

enum PipelineResult {
    Handled,
    Continue,
    ReplaceState(State),
    HandledPushState(State),
    PushStateKeepBuf(State, Option<char>),
    PopState,
    PopStateType(State),
    Err,
    ErrUnexpected,
    Void,
}

/** {@see Tokenizer@pipeline} */
enum Pipe {
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
    ExpectDirectiveValue,
    ParseDirectiveValue,
    ExpectDirectiveClose,

    ExpectEventListenerOpenStart,
    ParseEventListenerOpen,
    ExpectEventListenerIdentifierStart,
    ExpectEventListenerIdentifierColon,
    ParseEventListenerIdentifier,
    ExpectEventListenerHandlerStart,
    ParseEventListenerHandler,
    ExpectEventListenerClose,

    ExpectAttrColon,
    ExpectExpressionStart,
    ExpectAttrSeparator,
}
