pub mod directive;
mod state;
#[cfg(test)]
mod test;
pub(crate) mod token;

use crate::parser::tokenizer::directive::{DirectiveEnum, DirectiveValueType};
use crate::parser::tokenizer::token::{ExprToken, Operator, OperatorMatchResult};
use state::State;
use std::assert_matches;
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
    tokens: Vec<Token<'token>>,
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
                State::ParsedAttrColon => self.pipeline(vec![
                    Pipe::ExpectOperandStart,
                    Pipe::ExpectNegativeOperator,
                    Pipe::ExpectParenthesisOpen,
                ]),
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
                State::ParsingDirectiveValue => self.pipeline(vec![
                    Pipe::SetStateForDirectiveValue,
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectNegativeOperator,
                    Pipe::ExpectOperandStart,
                    Pipe::ExpectParenthesisOpen,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::DirectiveExprStart => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectNegativeOperator,
                    Pipe::ExpectOperandStart,
                    Pipe::ExpectParenthesisOpen,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExpectDirectiveClose => self.pipeline(vec![Pipe::ExpectDirectiveClose]),
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
                    Pipe::ExpectNegativeOperator,
                    Pipe::ExpectOperandStart,
                    Pipe::ExpectParenthesisOpen,
                ]),
                State::ExprParsingNumber => self.pipeline(vec![
                    Pipe::ParseNumber,
                    Pipe::ExpectSlash,
                    Pipe::ExpectOperatorStart,
                    Pipe::ExpectParenthesisClose,
                    Pipe::ExpectDot,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprParsedNumber => self.pipeline(vec![
                    Pipe::ExpectSlash,
                    Pipe::ExpectOperatorStart,
                    Pipe::ExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprParsingFractionalNumber => self.pipeline(vec![
                    Pipe::ParseFractionalNumber,
                    Pipe::ExpectSlash,
                    Pipe::ExpectOperatorStart,
                    Pipe::ExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprExpectStringStart => self.pipeline(vec![Pipe::ExpectStringStart]),
                State::ExprParsingString => self.pipeline(vec![Pipe::ParseString]),
                State::ExprParsedString => self.pipeline(vec![
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectConcatenationOperator,
                    Pipe::ExpectPropAccessor,
                    Pipe::ExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprParsingVarOrKeyword => self.pipeline(vec![
                    Pipe::ParseVarOrKeyword,
                    Pipe::ExpectSlash,
                    Pipe::ExpectPropAccessor,
                    Pipe::ExpectBracketOpen,
                    Pipe::ExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprParsedVarOrKeyword => self.pipeline(vec![
                    Pipe::ExpectSlash,
                    Pipe::ExpectPropAccessor,
                    Pipe::ExpectBracketOpen,
                    Pipe::ExpectParenthesisClose,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprParsingOperator => self.pipeline(vec![
                    Pipe::ParseOperator,
                    Pipe::ExpectSlash,
                    Pipe::ExpectOperandStart,
                ]),
                State::ExprParsedOperator => {
                    self.pipeline(vec![Pipe::ExpectCommentStart, Pipe::ExpectOperandStart])
                }
                State::ExprParsedBracketOpen => self.pipeline(vec![
                    Pipe::UNIMPLEMENTED,
                    Pipe::ExpectCommentStart,
                    Pipe::ExpectOperandStart,
                ]),
                State::ExprParsedParenthesisClose => self.pipeline(vec![
                    Pipe::ExpectSlash,
                    Pipe::ExpectOperatorStart,
                    Pipe::ExpectParenthesisClose,
                    Pipe::ExpectPropAccessor,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprResolvingSlash => {
                    self.pipeline(vec![Pipe::ResolveSlash, Pipe::ExpectOperandStart])
                }
                State::ExprResolvingDot => self.pipeline(vec![
                    Pipe::ResolveDot,
                    Pipe::ExpectOperandStart,
                    Pipe::ExpectFractionalNumberStart,
                    Pipe::ExpectAttrSeparator,
                    Pipe::ExpectBlockClose,
                    Pipe::ExpectDirectiveClose,
                ]),
                State::ExprParsedNegativeOperator => self.pipeline(vec![Pipe::ExpectNumberStart]),
            }
        }

        // All states should have been popped, and we should be back to start
        if self.state_history.len() != 1 && self.state() != &State::Start {
            self.err_eof();
        }

        // self.match_with_source();

        &self.tokens
    }

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
                Pipe::SetStateForDirectiveValue => self.set_state_for_directive_value(),
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
                Pipe::ExpectOperandStart => self.expect_operand_start(),
                Pipe::ExpectStringStart => self.expect_string_start(),
                Pipe::ExpectNumberStart => self.expect_number_start(),
                Pipe::ExpectParenthesisOpen => self.expect_parenthesis_open(),
                Pipe::ParseString => self.parse_string(),
                Pipe::ParseNumber => self.parse_number(),
                Pipe::ParseVarOrKeyword => self.parse_var_or_keyword(),
                Pipe::ParseOperator => self.parse_operator(),
                Pipe::ExpectPropAccessor => self.expect_prop_accessor(),
                Pipe::ExpectBracketAccessorOpen => self.expect_bracket_accessor_open(),
                Pipe::ExpectBracketAccessorClose => self.expect_bracket_accessor_close(),
                Pipe::ExpectNegativeOperator => self.expect_negative_operator(),
                Pipe::ExpectOperatorStart => self.expect_operator_start(),
                Pipe::ExpectParenthesisClose => self.expect_parenthesis_close(),
                Pipe::ExpectSlash => self.expect_slash(),
                Pipe::ExpectDot => self.expect_dot(),
                Pipe::ExpectBracketOpen => self.expect_bracket_open(),
                Pipe::ResolveDot => self.resolve_dot(),
                Pipe::ResolveSlash => self.resolve_slash(),
                Pipe::ExpectFractionalNumberStart => self.expect_fractional_number_start(),
                Pipe::ParseFractionalNumber => self.parse_fractional_number(),
                Pipe::ExpectConcatenationOperator => self.expect_concatenation_operator(),
            };

            match result {
                PipelineResult::Handled => return,
                PipelineResult::Continue => { /* Continue to next pipe or finish loop. */ }
                PipelineResult::Err(msg) => self.err_msg(msg),
                PipelineResult::ErrUnexpected => self.err_unexpected(None),
            };
        }

        // todo WARNING: ignoring all whitespace here may cause problems down the line?
        if !self.char.is_ascii_whitespace() {
            dbg!(&self.state_history);
            panic!(
                r#"Unexpected token "{}" at line: {}, column: {}. State: "{}"."#,
                self.char,
                self.line,
                self.column,
                self.state(),
            )
        }
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
            panic!(r#"Expected to pop "{state}", found "{last}" instead."#);
        }
        self.state_history.pop();
    }

    fn push_token(&mut self, r#type: TokenType<'token>) {
        self.tokens
            .push(Token::new(self.buf_start.expect("Should be set."), r#type));

        self.reset_buffer();
    }

    fn push_token_pop_state(&mut self, r#type: TokenType<'token>) {
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

    fn active_directive(&'token self) -> Option<&'token DirectiveEnum<'token>> {
        let mut directive_is_closed = false;
        for token in self.tokens.iter().rev() {
            if let TokenType::DirectiveClose = token.r#type() {
                directive_is_closed = true;
            } else if let TokenType::DirectiveIdentifier(_, directive) = token.r#type() {
                if directive_is_closed {
                    directive_is_closed = false;
                } else {
                    return Some(directive);
                }
            }
        }
        None
    }

    fn expect_identifier_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphabetic() {
            self.reset_buffer();
            self.buf.push(char);
            self.push_state(State::ParsingIdentifier, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn parse_identifier_char(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphanumeric() || char == '-' || char == '_' {
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
            self.push_state(State::ParsingLineComment, Some(char));
            return PipelineResult::Handled;
        }

        if char == '*' {
            self.pop_state();
            self.push_state(State::ParsingBlockComment, Some(char));
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
            } else if self.state() == &State::ExprParsedVarOrKeyword {
                self.pop_state_type(State::ExprParsedVarOrKeyword);
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

        self.push_token(TokenType::DirectiveIdentifier(
            self.buf.clone(),
            DirectiveEnum::from(self.buf.as_str()),
        ));
        self.pop_state_type(State::ParsingDirectiveIdentifier);
        self.push_state(State::ParsedDirectiveIdentifier, None);

        PipelineResult::Continue
    }

    fn expect_directive_colon(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ':' {
            self.reset_buffer();
            self.push_token(TokenType::DirectiveColon);
            let mut active_directive = self.active_directive();
            let directive = active_directive.as_mut().expect("Should be set.");
            match directive.value_type() {
                DirectiveValueType::Expression => self.push_state(State::DirectiveExprStart, None),
                DirectiveValueType::Custom => self.push_state(State::ParsingDirectiveValue, None),
                DirectiveValueType::None => self.push_state(State::ExpectDirectiveClose, None),
                DirectiveValueType::String => self.push_state(State::ExprExpectStringStart, None),
                DirectiveValueType::ForStatement => todo!(),
            }
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn set_state_for_directive_value(&mut self) -> PipelineResult {
        match self.active_directive().as_ref().unwrap().value_type() {
            DirectiveValueType::Expression => {
                self.push_state(State::DirectiveExprStart, None);
                PipelineResult::Continue
            }
            DirectiveValueType::String => {
                self.push_state(State::ExprExpectStringStart, None);
                PipelineResult::Continue
            }
            DirectiveValueType::Custom => unimplemented!(),
            DirectiveValueType::None => PipelineResult::Continue,
            DirectiveValueType::ForStatement => todo!(),
        }
    }

    fn expect_directive_close(&mut self) -> PipelineResult {
        let char = self.char;

        if char == ']' {
            // We must first ensure that if the directive is being
            // closed, that it is being closed in a valid state.
            let active_directive = self.active_directive();
            match active_directive
                .expect("No active directive found.")
                .value_type()
            {
                DirectiveValueType::Expression => match self.tokens.last().unwrap().r#type() {
                    TokenType::ExprToken(_) => {
                        self.reset_buffer();
                        self.push_token(TokenType::DirectiveClose);
                        if matches!(
                            self.state(),
                            State::ExprParsedNumber
                                | State::ExprParsedString
                                | State::ExprParsedVarOrKeyword
                                | State::ExprParsedParenthesisClose
                        ) {
                            self.pop_state();
                        }
                        self.pop_state_type(State::DirectiveExprStart);
                    }
                    TokenType::DirectiveColon => {
                        return PipelineResult::Err(
                            "Expected directive expression before close.".to_lowercase(),
                        );
                    }
                    TokenType::DirectiveIdentifier(_, directive) => {
                        if let DirectiveValueType::None = directive.value_type() {
                            self.reset_buffer();
                            self.push_token(TokenType::DirectiveClose);
                        } else {
                            return PipelineResult::Err(
                                "Expected directive expression before close.".to_lowercase(),
                            );
                        }
                    }
                    _ => {
                        return PipelineResult::Err(
                            "Directive closed in an invalid state.".to_owned(),
                        );
                    }
                },
                DirectiveValueType::Custom | DirectiveValueType::None => {
                    self.reset_buffer();
                    self.push_token(TokenType::DirectiveClose);
                }
                DirectiveValueType::String => {
                    self.reset_buffer();
                    self.push_token(TokenType::DirectiveClose);
                    self.pop_state_type(State::ExprParsedString);
                }
                DirectiveValueType::ForStatement => todo!(),
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
            dbg!(
                self.state(),
                self.buf_start.unwrap(),
                self.buf.len(),
                self.cursor
            );

            let last_token = self.tokens.last().unwrap();
            let has_space = last_token.start() + last_token.end() < self.cursor;

            if self.state() == &State::ParsedBlockIdentifier && !has_space {
                return PipelineResult::Err(
                    "Must have space between event listener start and block identifier.".to_owned(),
                );
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
            return PipelineResult::Handled;
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

            match self.state() {
                State::ExprParsedString => self.pop_state_type(State::ExprParsedString),
                State::ExprParsedNumber => self.pop_state_type(State::ExprParsedNumber),
                _ => {}
            }

            self.pop_state_type(State::ParsedAttrColon);

            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    // Expressions

    fn expect_operand_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_digit() {
            if self.state() == &State::ExprParsedOperator {
                self.pop_state_type(State::ExprParsedOperator);
            }
            self.reset_buffer();
            self.push_state(State::ExprParsingNumber, Some(char));
            return PipelineResult::Handled;
        } else if char.is_ascii_alphabetic() {
            if self.state() == &State::ExprParsedOperator {
                self.pop_state_type(State::ExprParsedOperator);
            }
            self.reset_buffer();
            self.push_state(State::ExprParsingVarOrKeyword, Some(char));
            return PipelineResult::Handled;
        } else if char == '"' {
            self.reset_buffer();
            self.push_state(State::ExprParsingString, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_string_start(&mut self) -> PipelineResult {
        let char = self.char;

        // todo support single quote
        if char == '"' {
            self.pop_state_type(State::ExprExpectStringStart);
            self.reset_buffer();
            self.push_state(State::ExprParsingString, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_number_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_digit() {
            if self.state() == &State::ExprParsedNegativeOperator {
                self.pop_state_type(State::ExprParsedNegativeOperator);
            }
            if self.state() == &State::ExprParsedOperator {
                self.pop_state_type(State::ExprParsedOperator);
            }
            self.reset_buffer();
            self.push_state(State::ExprParsingNumber, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_parenthesis_open(&mut self) -> PipelineResult {
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

    fn parse_string(&mut self) -> PipelineResult {
        let char = self.char;

        // todo support single quotes
        self.buf.push(char);

        if char == '"' {
            let mut trailing_backslash_count: usize = 0;
            for (i, c) in self.buf.chars().rev().enumerate() {
                if i == 0 {
                    continue;
                }
                if c == '\\' {
                    trailing_backslash_count += 1;
                } else {
                    break;
                }
            }

            if trailing_backslash_count.is_multiple_of(2) {
                self.push_token(TokenType::ExprToken(ExprToken::String(self.buf.clone())));
                self.pop_state_type(State::ExprParsingString);
                self.push_state(State::ExprParsedString, None);
            }
        }

        PipelineResult::Handled
    }

    fn parse_number(&mut self) -> PipelineResult {
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

    fn parse_var_or_keyword(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_alphanumeric() {
            self.buf.push(char);
            return PipelineResult::Handled;
        }

        match self.buf.to_lowercase().as_str() {
            "true" => {
                self.push_token(TokenType::ExprToken(ExprToken::True))
            }
            "false" => {
                self.push_token(TokenType::ExprToken(ExprToken::False))
            }
            "null" => {
                self.push_token(TokenType::ExprToken(ExprToken::Null))
            }
            "nan" => unimplemented!(),
            _ => {
                self.push_token(TokenType::ExprToken(
                    match self.tokens.last().unwrap().r#type() {
                        TokenType::ExprToken(ExprToken::PropAccessorDot) => {
                            ExprToken::PropAccessorVar(self.buf.clone())
                        }
                        _ => ExprToken::Var(self.buf.clone()),
                    }
                ));
            }
        }

        self.pop_state_type(State::ExprParsingVarOrKeyword);
        self.push_state(State::ExprParsedVarOrKeyword, None);

        PipelineResult::Continue
    }

    fn expect_bracket_accessor_open(&mut self) -> PipelineResult {
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

    fn expect_bracket_accessor_close(&mut self) -> PipelineResult {
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

    fn expect_negative_operator(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '-' {
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::Operator(Operator::Minus)));
            self.push_state(State::ExprParsedNegativeOperator, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_operator_start(&mut self) -> PipelineResult {
        let char = self.char;

        if Operator::is_operator_char(char) {
            self.reset_buffer();
            self.buf.push(char);

            return match Operator::get_match(&self.buf) {
                OperatorMatchResult::Matched(op) => {
                    if self.state() == &State::ExprParsedNumber {
                        self.pop_state_type(State::ExprParsedNumber);
                    }
                    self.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                    self.push_state(State::ExprParsedOperator, None);
                    PipelineResult::Handled
                }
                OperatorMatchResult::Matches(_) => {
                    if self.state() == &State::ExprParsedNumber {
                        self.pop_state_type(State::ExprParsedNumber);
                    }
                    self.push_state(State::ExprParsingOperator, None);
                    PipelineResult::Handled
                }
                OperatorMatchResult::None => PipelineResult::Err(format!(
                    r#""{}" does not match any operators or is not implemented"#,
                    self.buf
                )),
            };
        }

        PipelineResult::Continue
    }

    fn parse_operator(&mut self) -> PipelineResult {
        let char = self.char;

        if Operator::is_operator_char(char) {
            self.buf.push(char);

            match Operator::get_match(&self.buf) {
                OperatorMatchResult::Matched(op) => {
                    if self.state() == &State::ExprParsedNumber {
                        self.pop_state_type(State::ExprParsedNumber);
                    }
                    self.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                    self.push_state(State::ExprParsedOperator, None);
                    self.pop_state_type(State::ExprParsingOperator);
                    PipelineResult::Handled
                }
                OperatorMatchResult::Matches(_) => PipelineResult::Handled,
                OperatorMatchResult::None => PipelineResult::Err(format!(
                    r#""{}" does not match any operators or is not implemented"#,
                    self.buf
                )),
            }
        } else {
            match Operator::get_exact(&self.buf) {
                Some(op) => {
                    self.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                    self.pop_state_type(State::ExprParsingOperator);
                    self.push_state(State::ExprParsedOperator, None);
                    PipelineResult::Continue
                }
                None => PipelineResult::Err(format!(
                    r#""{}" does not match any operators or is not implemented"#,
                    self.buf
                )),
            }
        }
    }

    fn expect_prop_accessor(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '.' {
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::PropAccessorDot));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_parenthesis_close(&mut self) -> PipelineResult {
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

    fn expect_slash(&mut self) -> PipelineResult {
        let char = self.char;

        PipelineResult::Continue
    }

    fn expect_dot(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '.' {
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::UnresolvedDot));
            self.push_state(State::ExprResolvingDot, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn expect_bracket_open(&mut self) -> PipelineResult {
        let char = self.char;

        PipelineResult::Continue
    }

    fn resolve_dot(&mut self) -> PipelineResult {
        let char = self.char;
        assert_matches!(
            self.tokens.last().unwrap().r#type(),
            TokenType::ExprToken(ExprToken::UnresolvedDot)
        );

        if char.is_ascii_digit() || matches!(char, ',' | '}' | ')') {
            self.tokens.pop(); // Pop the unresolved dot
            let prev = self.tokens.pop().unwrap();
            if let TokenType::ExprToken(ExprToken::Number(num)) = prev.r#type() {
                let mut num = num.clone();
                num.push('.');
                self.tokens.push(Token::new(
                    prev.start(),
                    TokenType::ExprToken(ExprToken::Number(num)),
                ));
                self.pop_state_type(State::ExprResolvingDot);
                return if char.is_ascii_digit() {
                    self.push_state(State::ExprParsingFractionalNumber, Some(char));
                    PipelineResult::Handled
                } else {
                    PipelineResult::Continue
                };
            };
        }

        if char.is_ascii_alphabetic() {
            // Is a prop accessor dot
            self.tokens.pop(); // Pop the unresolved dot
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::PropAccessorDot));
            self.pop_state_type(State::ExprResolvingDot);
            self.push_state(State::ExprParsingVarOrKeyword, Some(char))
        }

        PipelineResult::Continue
    }

    fn resolve_slash(&mut self) -> PipelineResult {
        todo!();
        let char = self.char;

        PipelineResult::Continue
    }

    fn expect_fractional_number_start(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_digit() {
            self.reset_buffer();
            self.push_state(State::ExprParsingFractionalNumber, Some(char));
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
    }

    fn parse_fractional_number(&mut self) -> PipelineResult {
        let char = self.char;

        if char.is_ascii_digit() {
            self.buf.push(char);
            return PipelineResult::Handled;
        }

        let num_tok = self.tokens.pop().unwrap();
        if let TokenType::ExprToken(ExprToken::Number(num)) = num_tok.r#type() {
            self.tokens.push(Token::new(
                num_tok.start(),
                TokenType::ExprToken(ExprToken::Number(format!("{}{}", num, self.buf))),
            ));
            self.pop_state_type(State::ExprParsingFractionalNumber);
        }

        PipelineResult::Continue
    }

    fn expect_concatenation_operator(&mut self) -> PipelineResult {
        let char = self.char;

        if char == '+' {
            self.reset_buffer();
            self.push_token(TokenType::ExprToken(ExprToken::Operator(Operator::Plus)));
            self.pop_state_type(State::ExprParsedString);
            self.push_state(State::ExprParsedOperator, None);
            return PipelineResult::Handled;
        }

        PipelineResult::Continue
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
