use super::helper::{
    current, pop_state, pop_state_type, prev, push_state, push_state_keep_buf, push_token,
    reset_buffer,
};
use super::token::{ExprToken, Operator};
use super::{Token, TokenType, TokenizerContract};
use std::fmt::{Display, Formatter};

// pub(super) struct ExpressionTokenizer {
//     state_history: Vec<ExprState>,
//     tokens: Vec<Token>,
//     cursor: usize, // This is a reference to the position of the cursor for the source string/file
//     buf: String,
//     buf_start: Option<usize>,
// }
// 
// impl ExpressionTokenizer {
//     pub(super) fn new() -> ExpressionTokenizer {
//         ExpressionTokenizer {
//             state_history: vec![ExprState::Start],
//             tokens: vec![],
//             cursor: 0,
//             buf: String::new(),
//             buf_start: None,
//         }
//     }
// 
//     fn get_tokens_and_reset(&mut self) -> ExprResult {
//         self.reset_buffer();
//         self.state_history = vec![ExprState::Start];
//         let mut taken = Vec::new();
//         taken.append(&mut self.tokens);
//         ExprResult::Parsed(taken)
//     }
// 
//     fn err(&self, msg: String) -> ExprResult {
//         ExprResult::Err(msg)
//     }
// 
//     fn err_unexpected(&self) -> ExprResult {
//         ExprResult::ErrUnexpected(self.state_history.last().expect("Should be set"))
//     }
// 
//     fn err_expected_expr_start(&self) -> ExprResult {
//         ExprResult::ErrExpectedExprStart(format!(
//             r#"Expected ", alphabetic, or number. ExprState: "{}""#,
//             self.state()
//         ))
//     }
// 
//     fn is_expr_end(&self, char: char) -> bool {
//         char == ',' || char == '}'
//     }
// 
//     fn close_parenth_has_match(&mut self) -> bool {
//         let open_count = self
//             .tokens
//             .iter()
//             .filter(|tok| {
//                 matches!(
//                     tok.r#type(),
//                     TokenType::ExprToken(ExprToken::Operator(Operator::ParenthesesOpen))
//                 )
//             })
//             .count();
//         let close_count = self
//             .tokens
//             .iter()
//             .filter(|tok| {
//                 matches!(
//                     tok.r#type(),
//                     TokenType::ExprToken(ExprToken::Operator(Operator::ParenthesesClose))
//                 )
//             })
//             .count();
//         open_count <= close_count
//     }
// 
//     pub(super) fn push_parse(&mut self, cursor: usize, char: char) -> ExprResult {
//         self.cursor = cursor;
// 
//         // match self.state() {
//         //     ExprState::Start => self.pipeline(
//         //         vec![
//         //             ExprPipe::ExpectNumberStart,
//         //             ExprPipe::ExpectStringStart,
//         //             ExprPipe::ExpectParenthesisOpen,
//         //             ExprPipe::ExpectVarStart,
//         //         ]
//         //     ),
//         //     ExprState::ParsingNumber => self.pipeline(vec![]),
//         //     ExprState::ParsedNumber => self.pipeline(vec![]),
//         //     ExprState::ParsingString => self.pipeline(vec![]),
//         //     ExprState::ParsedString => self.pipeline(vec![]),
//         //     ExprState::ParsingVar => self.pipeline(vec![]),
//         //     ExprState::ParsedVar => self.pipeline(vec![]),
//         //     ExprState::ParsedFuncParenthesisOpen => self.pipeline(vec![]),
//         //     ExprState::ParsedFuncArg => self.pipeline(vec![]),
//         //     ExprState::ParsedParenthesisClose => self.pipeline(vec![]),
//         // }
// 
//         ExprResult::StillParsing
//     }
// }
// 
// impl TokenizerContract for ExpressionTokenizer {
//     type TokenizerState = ExprState;
//     type PipeType = ExprPipe;
// 
//     fn pipeline(&mut self, pipes: Vec<ExprPipe>) {
//         //
//     }
// 
//     fn state(&self) -> &ExprState {
//         current(&self.state_history)
//     }
// 
//     fn prev_state(&self) -> Option<&ExprState> {
//         prev(&self.state_history)
//     }
// 
//     fn push_state(&mut self, state: ExprState, starting_char: Option<char>) {
//         push_state(
//             &mut self.state_history,
//             state,
//             &mut self.buf,
//             starting_char,
//         );
//     }
// 
//     fn push_state_keep_buf(&mut self, state: Self::TokenizerState, starting_char: Option<char>) {
//         push_state_keep_buf(&mut self.state_history, state, &mut self.buf, starting_char);
//     }
// 
//     fn pop_state(&mut self) {
//         pop_state(&mut self.state_history);
//     }
// 
//     fn pop_state_type(&mut self, state: Self::TokenizerState) {
//         pop_state_type(&mut self.state_history, state);
//     }
// 
//     fn push_token(&mut self, r#type: TokenType) {
//         push_token(
//             &mut self.tokens,
//             match self.buf_start {
//                 Some(s) => s,
//                 None => panic!("buf_start not set. {}", r#type)
//             },
//             r#type,
//         );
//         self.reset_buffer();
//     }
// 
//     fn push_token_pop_state(&mut self, r#type: TokenType) {
//         self.push_token(r#type);
//         self.pop_state();
//     }
// 
//     fn buf_last(&self) -> Option<char> {
//         self.buf.chars().last()
//     }
// 
//     fn reset_buffer(&mut self) {
//         reset_buffer(&mut self.buf, &mut self.buf_start, self.cursor);
//     }
// }

#[derive(Debug, PartialEq)]
pub(super) enum ExprState {
    Start,
    ParsingNumber,
    ParsedNumber,
    ParsingString,
    ParsedString,
    ParsingVar,
    ParsedVar,
    ParsedFuncParenthesisOpen,
    ParsedFuncArg,
    ParsedParenthesisClose,
}

impl Display for ExprState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprState::Start => write!(f, "ExpectOperandStart"),

            ExprState::ParsingNumber => write!(f, "ParsingNumber"),
            ExprState::ParsedNumber => write!(f, "ParsedNumber"),
            ExprState::ParsingString => write!(f, "ParsingString"),
            ExprState::ParsedString => write!(f, "ParsedString"),
            ExprState::ParsingVar => write!(f, "ParsingVar"),
            ExprState::ParsedVar => write!(f, "ParsedVar"),
            ExprState::ParsedFuncParenthesisOpen => write!(f, "ParsedFuncParenthesisOpen"),
            ExprState::ParsedFuncArg => write!(f, "ParsedFuncArg"),
            ExprState::ParsedParenthesisClose => write!(f, "ParsedParenthesisClose"),
        }
    }
}

pub(super) enum ExprResult<'a> {
    StillParsing,
    Parsed(Vec<Token>),
    Err(String),
    ErrExpectedExprStart(String),
    ErrUnexpected(&'a ExprState),
}

enum ExprPipe {
    ExpectOperandStart,
    ExpectParenthesisOpen,
    ParseString,
    ParseNumber,
    ExpectIdentifierStart,
    ParseIdentifier,
    ExpectVarDotAccessor,
    ExpectVarBracketAccessorOpen,
    ExpectVarBracketAccessorClose,
    ExpectFunctionArgsOpen,
    ExpectFunctionArgsClose,
}
