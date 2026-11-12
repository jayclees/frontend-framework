mod helper;
mod state;
#[cfg(test)]
mod test;
pub(crate) mod token;

use crate::parser::tokenizer::token::{ExprToken, Operator, OperatorMatchResult};
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
            self.push_token(TokenType::BlockIdentifier(self.buf.clone()));
            self.pop_state_type(State::ParsingIdentifier);
            if self.state() == &State::ParsingIdentifier {
                self.pop_state_type(State::ParsingIdentifier);
            }
            self.push_state(State::ParsedBlockIdentifier, None);
        }

        if char == ':' {
            if self.tokens.is_empty() {
                return PipelineResult::ErrUnexpected;
            }
            self.push_token_pop_state(TokenType::AttrIdentifier(self.buf.clone()));
            self.push_state(State::ParsedAttrIdentifier, None);
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
            if self.state() == &State::ParsedBlockIdentifier {
                self.pop_state_type(State::ParsedBlockIdentifier);
            }
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

            if self.state() == &State::ExprParsedNumber {
                self.pop_state_type(State::ExprParsedNumber);
                self.pop_state_type(State::ParsedAttrColon);
            } else if self.state() == &State::ExprParsedString {
                self.pop_state_type(State::ExprParsedString);
                self.pop_state_type(State::ParsedAttrColon);
            } else if self.state() == &State::ExprParsedVar {
                self.pop_state_type(State::ExprParsedVar);
                self.pop_state_type(State::ParsedAttrColon);
            }

            // if any of these states are the previous ones, the tokenizer is in an invalid state
            // ExprParsedOperator
            // ExprParsedBracketOpen
            // ExprParsedFuncParenthesisOpen
            // ExprParsedFuncOperand
            // ExprParsedParenthesisClose

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
            self.push_state(State::ParsingDirectiveIdentifier, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn parse_directive_identifier(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphanumeric() {
            self.buf.push(char);
            return PipelineResult::Handled;
        }

        self.push_token(TokenType::DirectiveIdentifier(self.buf.clone()));
        self.pop_state_type(State::ParsingDirectiveIdentifier);
        self.push_state(State::ParsedDirectiveIdentifier, None);

        PipelineResult::Continue
    }

    fn expect_directive_colon(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ':' {
            self.reset_buffer();
            self.push_token(TokenType::DirectiveColon);
            self.push_state(State::ParsedDirectiveColon, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_directive_value_start(&mut self) -> PipelineResult {
        unimplemented!();
        // let char = self.char;
        //
        // // todo for now we are just looking for a ] and closing it.
        // // todo finish implementation later
        // if char == ']' {
        //     self.push_token(TokenType::DirectiveValue(self.buf.clone()));
        //     self.pop_state_type(State::ParsedDirectiveColon);
        //     return PipelineResult::Continue;
        // }
        //
        // self.buf.push(char);
        //
        // PipelineResult::Handled
    }

    fn parse_directive_value(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ']' {
            if self.buf.is_empty() {
                return PipelineResult::ErrUnexpected;
            }

            self.push_token(TokenType::DirectiveValue(self.buf.clone()));
            return PipelineResult::Continue;
        }

        if self.buf.is_empty() {
            self.reset_buffer();
        }
        self.buf.push(char);

        PipelineResult::Handled
    }

    fn expect_directive_close(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ']' {
            // todo ensure at least one character for directive value for testing

            self.reset_buffer();
            self.push_token(TokenType::DirectiveClose);
            if self.state() == &State::ParsedDirectiveColon {
                self.pop_state_type(State::ParsedDirectiveColon);
            }
            self.pop_state_type(State::ParsedDirectiveIdentifier);
            self.pop_state_type(State::ParsingDirective);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_event_listener_open_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '@' {
            // must have whitespace between identifier and @ symbol
            dbg!(self.state(), self.buf_start.unwrap(), self.buf.len(), self.cursor);

            let last_token = self.tokens.last().unwrap();
            let has_space = last_token.start() + last_token.end() < self.cursor;

            if self.state() == &State::ParsedBlockIdentifier && !has_space {
                return PipelineResult::Err("Must have space between event listener start and block identifier.".to_owned());
            }

            self.reset_buffer();
            self.push_state(State::ParsingEventListenerOpen, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn parse_event_listener_open(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '[' {
            self.push_token(TokenType::EventListenerOpen);
            self.pop_state_type(State::ParsingEventListenerOpen);
            self.push_state(State::ParsedEventListenerOpen, None);
            return PipelineResult::Handled;
        }

        PipelineResult::ErrUnexpected
    }

    fn expect_event_listener_identifier_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphabetic() {
            self.reset_buffer();
            self.push_state(State::ParsingEventListenerIdentifier, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn parse_event_listener_identifier(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphanumeric() {
            self.buf.push(char);
            return PipelineResult::Handled;
        }

        self.push_token(TokenType::EventListenerIdentifier(self.buf.clone()));
        self.pop_state_type(State::ParsingEventListenerIdentifier);
        self.push_state(State::ParsedEventListenerIdentifier, None);

        PipelineResult::Continue
    }

    fn expect_event_listener_identifier_colon(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ':' {
            self.reset_buffer();
            self.push_token(TokenType::EventListenerColon);
            self.push_state(State::ParsedEventListenerColon, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_event_listener_handler_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphabetic() {
            self.pop_state_type(State::ParsedEventListenerColon);
            self.reset_buffer();
            self.push_state(State::ParsingEventListenerHandler, Some(char));
            return PipelineResult::Handled
        }

        PipelineResult::Continue
    }

    fn parse_event_listener_handler(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ']' {
            self.push_token(TokenType::EventListenerHandler(self.buf.clone()));
            self.pop_state_type(State::ParsingEventListenerHandler);
            return PipelineResult::Continue;
        }

        self.buf.push(char);

        PipelineResult::Handled

    }

    fn expect_event_listener_close(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ']' {
            self.reset_buffer();
            self.push_token(TokenType::EventListenerClose);
            self.pop_state_type(State::ParsedEventListenerIdentifier);
            self.pop_state_type(State::ParsedEventListenerOpen);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_attr_colon(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ':' {
            self.reset_buffer();
            self.push_token(TokenType::AttrColon);
            self.pop_state_type(State::ParsedAttrIdentifier);
            self.push_state(State::ParsedAttrColon, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_attr_separator(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ',' {
            self.reset_buffer();
            self.push_token(TokenType::AttrSeparator);

            if self.state() == &State::ExprParsedString {
                self.pop_state_type(State::ExprParsedString);
            }

            self.pop_state_type(State::ParsedAttrColon);

            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    // Expressions

    fn expr_expect_operand_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_digit() {
            self.reset_buffer();
            self.push_state(State::ExprParsingNumber, Some(char));
            return PipelineResult::Handled;
        } else if char.is_ascii_alphabetic() {
            self.reset_buffer();
            self.push_state(State::ExprParsingVar, Some(char));
            return PipelineResult::Handled;
        } else if char == '"' {
            self.reset_buffer();
            self.push_state(State::ExprParsingString, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expr_expect_parenthesis_open(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '(' {
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::Operator(
                Operator::ParenthesesOpen,
            )));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expr_parse_string(&mut self) -> PipelineResult {
        let char = self.char;

        // todo support single quotes
        self.buf.push(char);

        if char == '"' {
            let trailing = self.buf.rsplit_once('\\');
            let trailing_backslash_count = trailing.iter().len();

            if trailing_backslash_count % 2 == 0 {
                self.push_token(TokenType::ExprToken(ExprToken::String(self.buf.clone())));
                self.pop_state_type(State::ExprParsingString);
                self.push_state(State::ExprParsedString, None);
            }
        }

        PipelineResult::Handled
    }

    fn expr_parse_number(&mut self) -> PipelineResult {
        let char = self.char;

        if self.char.is_ascii_digit() {
            self.buf.push(char);
            return PipelineResult::Handled;
        }

        self.push_token(TokenType::ExprToken(ExprToken::Number(self.buf.clone())));
        self.pop_state_type(State::ExprParsingNumber);
        self.push_state(State::ExprParsedNumber, None);

        PipelineResult::Continue
    }

    fn expr_parse_var(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphanumeric() {
            self.buf.push(char);
            return PipelineResult::Handled;
        }

        self.push_token(TokenType::ExprToken(ExprToken::Var(self.buf.clone())));
        self.pop_state_type(State::ExprParsingVar);
        self.push_state(State::ExprParsedVar, None);

        PipelineResult::Continue
    }

    fn expr_expect_bracket_accessor_open(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '[' {
            // todo check previous token to see what type of bracket this is.
            // todo array declaration? array access?
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::BracketAccessorOpen));
            self.push_state(State::ExprParsedBracketOpen, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expr_expect_bracket_accessor_close(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ']' {
            // todo check most recent bracket type to see what type of bracket this is.
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::BracketAccessorClose));
            self.pop_state_type(State::ExprParsedBracketOpen);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expr_expect_negative_operator(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '-' {
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::BracketAccessorClose));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expr_expect_operator_start(&mut self) -> PipelineResult {
        let char = self.char;

        if Operator::is_operator_char(char) {
            self.buf.push(char);

            return match Operator::get_match(&self.buf) {
                OperatorMatchResult::Matched(op) => {
                    self.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                    self.push_state(State::ExprParsedOperator, None);
                    PipelineResult::Handled
                }
                OperatorMatchResult::Matches(_) => {
                    self.push_state(State::ExprParsingOperator, None);
                    PipelineResult::Handled
                }
                OperatorMatchResult::None => PipelineResult::Err(format!(
                    r#""{}" does not match any operators or is not implemented"#,
                    &self.buf
                )),
            };
        }

        match Operator::get_exact(&self.buf) {
            Some(op) => {
                self.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                self.push_state(State::ExprParsedOperator, None);
                PipelineResult::Handled
            }
            None => PipelineResult::Err(format!(
                r#""{}" does not match any operators or is not implemented"#,
                &self.buf
            )),
        }
    }

    fn expr_parse_operator(&mut self) -> PipelineResult {
        let char = self.char;

        if Operator::is_operator_char(char) {
            self.buf.push(char);

            return match Operator::get_match(&self.buf) {
                OperatorMatchResult::Matched(op) => {
                    self.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                    self.push_state(State::ExprParsedOperator, None);
                    self.pop_state_type(State::ExprParsingOperator);
                    PipelineResult::Handled
                }
                OperatorMatchResult::Matches(_) => PipelineResult::Handled,
                OperatorMatchResult::None => PipelineResult::Err(format!(
                    r#""{}" does not match any operators or is not implemented"#,
                    &self.buf
                )),
            };
        }

        PipelineResult::ErrUnexpected
    }

    fn expr_expect_parenthesis_close(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ')' {
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::Operator(
                Operator::ParenthesesClose,
            )));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expr_expect_slash(&mut self) -> PipelineResult {
        let char = self.char;

        PipelineResult::Continue
    }

    fn expr_expect_dot(&mut self) -> PipelineResult {
        let char = self.char;

        PipelineResult::Continue
    }

    fn expr_expect_bracket_open(&mut self) -> PipelineResult {
        let char = self.char;

        PipelineResult::Continue
    }

    fn expr_resolve_dot(&mut self) -> PipelineResult {
        let char = self.char;

        PipelineResult::Continue
    }

    fn expr_resolve_slash(&mut self) -> PipelineResult {
        let char = self.char;

        PipelineResult::Continue
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
                    Pipe::ExpectAttrColon,
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
                    // Order should matter here? ExpectCommentStart
                    // must be before ExpectExpressionStart.
                    self.pipeline(vec![Pipe::ExprExpectOperandStart])
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
                State::ParsedDirectiveIdentifier => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectDirectiveColon,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ParsedDirectiveColon => {
                    self.pipeline(vec![Pipe::ParseDirectiveValue, Pipe::ExpectDirectiveClose])
                }
                State::ParsingDirectiveValue => {
                    self.pipeline(vec![Pipe::ParseDirectiveValue, Pipe::ExpectDirectiveClose])
                }
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
                State::ParsedEventListenerIdentifier => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectEventListenerIdentifierColon,
                ]),
                State::ParsedEventListenerColon => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectEventListenerHandlerStart,
                ]),
                State::ParsingEventListenerHandler => self.pipeline(vec![
                    Pipe::ParseEventListenerHandler,
                    Pipe::ExpectEventListenerClose,
                ]),
                State::ExprStart => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExprExpectNegativeOperator,
                    Pipe::ExprExpectOperandStart,
                    Pipe::ExprExpectParenthesisOpen,
                ]),
                // todo if current char is not a number in the following state,
                // todo make sure to pop ExprParsingNumber state in handler
                State::ExprParsingNumber => self.pipeline(vec![
                    Pipe::ExprParseNumber,
                    Pipe::ExprExpectOperatorStart,
                    Pipe::ExprExpectParenthesisClose,
                    Pipe::ExprExpectDot,
                    Pipe::ExprExpectSlash,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                ]),
                State::ExprParsedNumber => self.pipeline(vec![
                    Pipe::ExprExpectSlash,
                    Pipe::ExprExpectOperatorStart,
                    Pipe::ExprExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                ]),
                State::ExprParsingString => self.pipeline(vec![Pipe::ExprParseString]),
                State::ExprParsedString => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExprExpectDot,
                    Pipe::ExprExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                ]),
                State::ExprParsingVar => self.pipeline(vec![
                    Pipe::ExprParseVar,
                    Pipe::ExprExpectSlash,
                    Pipe::ExprExpectDot,
                    Pipe::ExprExpectBracketOpen,
                    Pipe::ExprExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                ]),
                State::ExprParsedVar => self.pipeline(vec![
                    Pipe::ExprExpectSlash,
                    Pipe::ExprExpectDot,
                    Pipe::ExprExpectBracketOpen,
                    Pipe::ExprExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                ]),
                State::ExprParsingOperator => self.pipeline(vec![Pipe::ExprParseOperator]),
                State::ExprParsedOperator => {
                    self.pipeline(vec![Pipe::ExpectCommentStart, Pipe::ExprExpectOperandStart])
                }
                State::ExprParsedBracketOpen => self.pipeline(vec![
                    Pipe::UNIMPLEMENTED,
                    Pipe::ExpectCommentStart,
                    Pipe::ExprExpectOperandStart,
                ]),
                State::ExprParsedParenthesisClose => self.pipeline(vec![
                    Pipe::ExprExpectSlash,
                    Pipe::ExprExpectOperatorStart,
                    Pipe::ExprExpectParenthesisClose,
                    Pipe::ExprExpectDot,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                ]),
                State::ExprResolveSlash => {
                    self.pipeline(vec![Pipe::ExprResolveSlash, Pipe::ExprExpectOperandStart])
                }
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
    type PipeType = Pipe;

    fn pipeline(&mut self, pipes: Vec<Pipe>) {
        for pipe in pipes {
            let result = match pipe {
                Pipe::UNIMPLEMENTED => unimplemented!(),
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
                Pipe::ExpectDirectiveValueStart => self.expect_directive_value_start(),
                Pipe::ParseDirectiveValue => self.parse_directive_value(),
                Pipe::ExpectDirectiveClose => self.expect_directive_close(),
                Pipe::ExpectEventListenerOpenStart => self.expect_event_listener_open_start(),
                Pipe::ParseEventListenerOpen => self.parse_event_listener_open(),
                Pipe::ExpectEventListenerIdentifierStart => {
                    self.expect_event_listener_identifier_start()
                }
                Pipe::ParseEventListenerIdentifier => self.parse_event_listener_identifier(),
                Pipe::ExpectEventListenerIdentifierColon => {
                    self.expect_event_listener_identifier_colon()
                }
                Pipe::ExpectEventListenerHandlerStart => self.expect_event_listener_handler_start(),
                Pipe::ParseEventListenerHandler => self.parse_event_listener_handler(),
                Pipe::ExpectEventListenerClose => self.expect_event_listener_close(),
                Pipe::ExpectAttrColon => self.expect_attr_colon(),
                Pipe::ExpectAttrSeparator => self.expect_attr_separator(),

                // Expressions
                Pipe::ExprExpectOperandStart => self.expr_expect_operand_start(),
                Pipe::ExprExpectParenthesisOpen => self.expr_expect_parenthesis_open(),
                Pipe::ExprParseString => self.expr_parse_string(),
                Pipe::ExprParseNumber => self.expr_parse_number(),
                Pipe::ExprParseVar => self.expr_parse_var(),
                Pipe::ExprParseOperator => self.expr_parse_operator(),
                Pipe::ExprExpectBracketAccessorOpen => self.expr_expect_bracket_accessor_open(),
                Pipe::ExprExpectBracketAccessorClose => self.expr_expect_bracket_accessor_close(),
                Pipe::ExprExpectNegativeOperator => self.expr_expect_negative_operator(),
                Pipe::ExprExpectOperatorStart => self.expr_expect_operator_start(),
                Pipe::ExprExpectParenthesisClose => self.expr_expect_parenthesis_close(),
                Pipe::ExprExpectSlash => self.expr_expect_slash(),
                Pipe::ExprExpectDot => self.expr_expect_dot(),
                Pipe::ExprExpectBracketOpen => self.expr_expect_bracket_open(),
                Pipe::ExprResolveDot => self.expr_resolve_dot(),
                Pipe::ExprResolveSlash => self.expr_resolve_slash(),
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
                PipelineResult::Err(msg) => self.err_msg(msg),
                PipelineResult::ErrUnexpected => self.err_unexpected(None),
            };
        }

        // todo WARNING: ignoring all whitespace here may cause problems down the line?
        if !self.char.is_ascii_whitespace() {
            panic!(
                r#"Unexpected token "{}" at line: {}, column: {}. State: "{}"."#,
                self.char,
                self.line,
                self.column,
                self.state(),
            )
        }
    }

    fn state(&self) -> &State {
        current(&self.state_history)
    }

    fn prev_state(&self) -> Option<&State> {
        prev(&self.state_history)
    }

    fn push_state(&mut self, state: State, starting_char: Option<char>) {
        push_state(&mut self.state_history, state, &mut self.buf, starting_char);
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

#[derive(Debug)]
enum PipelineResult {
    Handled,
    Continue,
    ReplaceState(State),
    HandledPushState(State),
    PushStateKeepBuf(State, Option<char>),
    PopState,
    PopStateType(State),
    Err(String),
    ErrUnexpected,
    Void,
}

/** {@see Tokenizer@pipeline} */
pub enum Pipe {
    // Dummy pipe that will trigger "unimplemented" to mark unhandled tokenizer states
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
    ExpectDirectiveValueStart,
    ParseDirectiveValue,
    ExpectDirectiveClose,

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
    ExprExpectOperandStart,
    ExprExpectParenthesisOpen,
    ExprParseString,
    ExprParseNumber,
    ExprParseVar,
    ExprParseOperator,
    ExprExpectBracketAccessorOpen,
    ExprExpectBracketAccessorClose,
    // ExprExpectFunctionArgsOpen,
    // ExprExpectFunctionArgsClose,
    ExprExpectNegativeOperator,
    ExprExpectOperatorStart,
    ExprExpectParenthesisClose,
    ExprExpectSlash,
    ExprExpectDot,
    ExprExpectBracketOpen,
    ExprResolveDot,
    ExprResolveSlash,
}
