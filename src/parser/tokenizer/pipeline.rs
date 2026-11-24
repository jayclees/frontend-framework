use crate::parser::tokenizer::directive::{DirectiveEnum, DirectiveValueType};
use crate::parser::tokenizer::state::State;
use crate::parser::tokenizer::token::{ExprToken, Operator, OperatorMatchResult, Token, TokenType};
use crate::parser::tokenizer::{Pipe, PipelineResult, Tokenizer};
use std::assert_matches;

pub(super) fn pipeline(tokenizer: &mut Tokenizer, pipes: Vec<Pipe>) {
    for pipe in pipes {
        let result = match pipe {
            Pipe::UNIMPLEMENTED => unimplemented!(),
            Pipe::ExpectIdentifierStart => expect_identifier_start(tokenizer),
            Pipe::ParseIdentifierChar => parse_identifier_char(tokenizer),
            Pipe::ResolveIdentifierType => resolve_identifier_type(tokenizer),
            Pipe::ExpectCommentStart => expect_comment_start(tokenizer),
            Pipe::ResolveCommentType => resolve_comment_type(tokenizer),
            Pipe::ParseLineComment => parse_line_comment(tokenizer),
            Pipe::ParseBlockComment => parse_block_comment(tokenizer),
            Pipe::ExpectBlockOpen => expect_block_open(tokenizer),
            Pipe::ExpectBlockClose => expect_block_close(tokenizer),
            Pipe::ExpectDirectiveOpen => expect_directive_open(tokenizer),
            Pipe::ExpectDirectiveIdentifierStart => expect_directive_identifier_start(tokenizer),
            Pipe::ParseDirectiveIdentifier => parse_directive_identifier(tokenizer),
            Pipe::ExpectDirectiveColon => expect_directive_colon(tokenizer),
            Pipe::SetStateForDirectiveValue => set_state_for_directive_value(tokenizer),
            Pipe::ExpectDirectiveClose => expect_directive_close(tokenizer),
            Pipe::ExpectEventListenerOpenStart => expect_event_listener_open_start(tokenizer),
            Pipe::ParseEventListenerOpen => parse_event_listener_open(tokenizer),
            Pipe::ExpectEventListenerIdentifierStart => {
                expect_event_listener_identifier_start(tokenizer)
            }
            Pipe::ParseEventListenerIdentifier => parse_event_listener_identifier(tokenizer),
            Pipe::ExpectEventListenerIdentifierColon => {
                expect_event_listener_identifier_colon(tokenizer)
            }
            Pipe::ExpectEventListenerHandlerStart => expect_event_listener_handler_start(tokenizer),
            Pipe::ParseEventListenerHandler => parse_event_listener_handler(tokenizer),
            Pipe::ExpectEventListenerClose => expect_event_listener_close(tokenizer),
            Pipe::ExpectAttrColon => expect_attr_colon(tokenizer),
            Pipe::ExpectAttrSeparator => expect_attr_separator(tokenizer),

            // Directive values
            Pipe::ParseForDirectiveValue => parse_for_directive_value(tokenizer),

            // Expressions
            Pipe::ExpectOperandStart => expect_operand_start(tokenizer),
            Pipe::ExpectStringStart => expect_string_start(tokenizer),
            Pipe::ExpectNumberStart => expect_number_start(tokenizer),
            Pipe::ExpectParenthesisOpen => expect_parenthesis_open(tokenizer),
            Pipe::ParseString => parse_string(tokenizer),
            Pipe::ParseNumber => parse_number(tokenizer),
            Pipe::ParseVarOrKeyword => parse_var_or_keyword(tokenizer),
            Pipe::ParseOperator => parse_operator(tokenizer),
            Pipe::ExpectPropAccessor => expect_prop_accessor(tokenizer),
            Pipe::ExpectBracketAccessorOpen => expect_bracket_accessor_open(tokenizer),
            Pipe::ExpectBracketAccessorClose => expect_bracket_accessor_close(tokenizer),
            Pipe::ExpectNegativeOperator => expect_negative_operator(tokenizer),
            Pipe::ExpectOperatorStart => expect_operator_start(tokenizer),
            Pipe::ExpectParenthesisClose => expect_parenthesis_close(tokenizer),
            Pipe::ExpectSlash => expect_slash(tokenizer),
            Pipe::ExpectDot => expect_dot(tokenizer),
            Pipe::ExpectBracketOpen => expect_bracket_open(tokenizer),
            Pipe::ResolveDot => resolve_dot(tokenizer),
            Pipe::ResolveSlash => resolve_slash(tokenizer),
            Pipe::ExpectFractionalNumberStart => expect_fractional_number_start(tokenizer),
            Pipe::ParseFractionalNumber => parse_fractional_number(tokenizer),
            Pipe::ExpectConcatenationOperator => expect_concatenation_operator(tokenizer),
        };

        match result {
            PipelineResult::Handled => return,
            PipelineResult::Continue => { /* Continue to next pipe or finish loop. */ }
            PipelineResult::Err(msg) => tokenizer.err_msg(msg),
            PipelineResult::ErrUnexpected => tokenizer.err_unexpected(None),
        };
    }

    // todo WARNING: ignoring all whitespace here may cause problems down the line?
    if !tokenizer.char.is_ascii_whitespace() {
        dbg!(&tokenizer.state_history);
        panic!(
            r#"Unexpected token "{}" at line: {}, column: {}. State: "{}"."#,
            tokenizer.char,
            tokenizer.line,
            tokenizer.column,
            tokenizer.state(),
        )
    }
}

pub fn expect_identifier_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphabetic() {
        tokenizer.reset_buffer();
        tokenizer.buf.push(char);
        tokenizer.push_state(State::ParsingIdentifier, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_identifier_char(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphanumeric() || char == '-' || char == '_' {
        tokenizer.buf.push(char);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn resolve_identifier_type(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if matches!(char, '[' | '@' | '{') {
        tokenizer.push_token(TokenType::BlockIdentifier(tokenizer.buf.clone()));
        tokenizer.pop_state_type(State::ParsingIdentifier);
        if tokenizer.state() == &State::ParsingIdentifier {
            tokenizer.pop_state_type(State::ParsingIdentifier);
        }
        tokenizer.push_state(State::ParsedBlockIdentifier, None);
    }

    if char == ':' {
        if tokenizer.tokens.is_empty() {
            return PipelineResult::ErrUnexpected;
        }
        tokenizer.push_token_pop_state(TokenType::AttrIdentifier(tokenizer.buf.clone()));
        tokenizer.push_state(State::ParsedAttrIdentifier, None);
    }

    PipelineResult::Continue
}

fn expect_comment_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '/' {
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ResolvingCommentType, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn resolve_comment_type(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '/' {
        tokenizer.pop_state();
        tokenizer.push_state(State::ParsingLineComment, Some(char));
        return PipelineResult::Handled;
    }

    if char == '*' {
        tokenizer.pop_state();
        tokenizer.push_state(State::ParsingBlockComment, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::ErrUnexpected
}

fn parse_line_comment(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '\n' {
        tokenizer.buf.push(char);
        tokenizer.push_token_pop_state(TokenType::LineComment(tokenizer.buf.clone()));
    } else {
        tokenizer.buf.push(char);
    }

    PipelineResult::Handled
}

fn parse_block_comment(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if let Some(last) = tokenizer.buf.chars().last()
        && last == '*'
        && char == '/'
    {
        tokenizer.buf.push(char);
        tokenizer.push_token(TokenType::BlockComment(tokenizer.buf.clone()));
        tokenizer.pop_state_type(State::ParsingBlockComment);
    } else {
        tokenizer.buf.push(char);
    }

    PipelineResult::Handled
}

fn expect_block_open(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '{' {
        tokenizer.reset_buffer();
        if tokenizer.state() == &State::ParsedBlockIdentifier {
            tokenizer.pop_state_type(State::ParsedBlockIdentifier);
        }
        tokenizer.push_token(TokenType::BlockOpen);
        tokenizer.push_state(State::ParsingBlock, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_block_close(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '}' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::BlockClose);

        if tokenizer.state() == &State::ExprParsedNumber {
            tokenizer.pop_state_type(State::ExprParsedNumber);
            tokenizer.pop_state_type(State::ParsedAttrColon);
        } else if tokenizer.state() == &State::ExprParsedString {
            tokenizer.pop_state_type(State::ExprParsedString);
            tokenizer.pop_state_type(State::ParsedAttrColon);
        } else if tokenizer.state() == &State::ExprParsedVarOrKeyword {
            tokenizer.pop_state_type(State::ExprParsedVarOrKeyword);
            tokenizer.pop_state_type(State::ParsedAttrColon);
        }

        tokenizer.pop_state_type(State::ParsingBlock);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_directive_open(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '[' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::DirectiveOpen);
        tokenizer.push_state(State::ParsingDirective, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_directive_identifier_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphabetic() {
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ParsingDirectiveIdentifier, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_directive_identifier(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphanumeric() {
        tokenizer.buf.push(char);
        return PipelineResult::Handled;
    }

    tokenizer.active_directive = Some(DirectiveEnum::from(tokenizer.buf.as_ref()));
    tokenizer.push_token(TokenType::DirectiveIdentifier(tokenizer.buf.clone()));
    tokenizer.pop_state_type(State::ParsingDirectiveIdentifier);
    tokenizer.push_state(State::ParsedDirectiveIdentifier, None);

    PipelineResult::Continue
}

fn expect_directive_colon(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ':' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::DirectiveColon);
        match tokenizer.active_directive.as_ref().unwrap().value_type() {
            DirectiveValueType::Expression => tokenizer.push_state(State::DirectiveExprStart, None),
            DirectiveValueType::Custom => tokenizer.push_state(State::ParsingDirectiveValue, None),
            DirectiveValueType::None => tokenizer.push_state(State::ExpectDirectiveClose, None),
            DirectiveValueType::String => tokenizer.push_state(State::ExprExpectStringStart, None),
            DirectiveValueType::ForStatement => todo!(),
        }
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn set_state_for_directive_value(tokenizer: &mut Tokenizer) -> PipelineResult {
    match tokenizer.active_directive.as_ref().unwrap().value_type() {
        DirectiveValueType::Expression => {
            tokenizer.push_state(State::DirectiveExprStart, None);
            PipelineResult::Continue
        }
        DirectiveValueType::String => {
            tokenizer.push_state(State::ExprExpectStringStart, None);
            PipelineResult::Continue
        }
        DirectiveValueType::Custom => unimplemented!(),
        DirectiveValueType::None => PipelineResult::Continue,
        DirectiveValueType::ForStatement => todo!(),
    }
}

fn expect_directive_close(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ']' {
        // We must first ensure that if the directive is being
        // closed, that it is being closed in a valid state.
        let active_directive = tokenizer
            .active_directive
            .as_ref()
            .expect("No active directive found.");
        match active_directive.value_type() {
            DirectiveValueType::Expression => match tokenizer.tokens.last().unwrap().r#type() {
                TokenType::ExprToken(_) => {
                    tokenizer.reset_buffer();
                    tokenizer.push_token(TokenType::DirectiveClose);
                    if matches!(
                        tokenizer.state(),
                        State::ExprParsedNumber
                            | State::ExprParsedString
                            | State::ExprParsedVarOrKeyword
                            | State::ExprParsedParenthesisClose
                    ) {
                        tokenizer.pop_state();
                    }
                    tokenizer.pop_state_type(State::DirectiveExprStart);
                }
                TokenType::DirectiveColon => {
                    return PipelineResult::Err(
                        "Expected directive expression before close.".to_lowercase(),
                    );
                }
                TokenType::DirectiveIdentifier(_) => {
                    if let DirectiveValueType::None = active_directive.value_type() {
                        tokenizer.reset_buffer();
                        tokenizer.push_token(TokenType::DirectiveClose);
                        // todo instead of dropping active_directive and recreating it later
                        // todo during token building, attach it to DirectiveIdentifier enum?
                        tokenizer.active_directive = None;
                    } else {
                        return PipelineResult::Err(
                            "Expected directive expression before close.".to_lowercase(),
                        );
                    }
                }
                _ => {
                    return PipelineResult::Err("Directive closed in an invalid state.".to_owned());
                }
            },
            DirectiveValueType::Custom | DirectiveValueType::None => {
                tokenizer.reset_buffer();
                tokenizer.push_token(TokenType::DirectiveClose);
            }
            DirectiveValueType::String => {
                tokenizer.reset_buffer();
                tokenizer.push_token(TokenType::DirectiveClose);
                tokenizer.pop_state_type(State::ExprParsedString);
            }
            DirectiveValueType::ForStatement => todo!(),
        }

        tokenizer.pop_state_type(State::ParsedDirectiveIdentifier);
        tokenizer.pop_state_type(State::ParsingDirective);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_event_listener_open_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '@' {
        // must have whitespace between identifier and @ symbol
        dbg!(
            tokenizer.state(),
            tokenizer.buf_start.unwrap(),
            tokenizer.buf.len(),
            tokenizer.cursor
        );

        let last_token = tokenizer.tokens.last().unwrap();
        let has_space = last_token.start() + last_token.end() < tokenizer.cursor;

        if tokenizer.state() == &State::ParsedBlockIdentifier && !has_space {
            return PipelineResult::Err(
                "Must have space between event listener start and block identifier.".to_owned(),
            );
        }

        tokenizer.reset_buffer();
        tokenizer.push_state(State::ParsingEventListenerOpen, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_event_listener_open(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '[' {
        tokenizer.push_token(TokenType::EventListenerOpen);
        tokenizer.pop_state_type(State::ParsingEventListenerOpen);
        tokenizer.push_state(State::ParsedEventListenerOpen, None);
        return PipelineResult::Handled;
    }

    PipelineResult::ErrUnexpected
}

fn expect_event_listener_identifier_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphabetic() {
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ParsingEventListenerIdentifier, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_event_listener_identifier(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphanumeric() {
        tokenizer.buf.push(char);
        return PipelineResult::Handled;
    }

    tokenizer.push_token(TokenType::EventListenerIdentifier(tokenizer.buf.clone()));
    tokenizer.pop_state_type(State::ParsingEventListenerIdentifier);
    tokenizer.push_state(State::ParsedEventListenerIdentifier, None);

    PipelineResult::Continue
}

fn expect_event_listener_identifier_colon(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ':' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::EventListenerColon);
        tokenizer.push_state(State::ParsedEventListenerColon, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_event_listener_handler_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphabetic() {
        tokenizer.pop_state_type(State::ParsedEventListenerColon);
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ParsingEventListenerHandler, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_event_listener_handler(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ']' {
        tokenizer.push_token(TokenType::EventListenerHandler(tokenizer.buf.clone()));
        tokenizer.pop_state_type(State::ParsingEventListenerHandler);
        return PipelineResult::Continue;
    }

    tokenizer.buf.push(char);

    PipelineResult::Handled
}

fn expect_event_listener_close(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ']' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::EventListenerClose);
        tokenizer.pop_state_type(State::ParsedEventListenerIdentifier);
        tokenizer.pop_state_type(State::ParsedEventListenerOpen);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_attr_colon(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ':' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::AttrColon);
        tokenizer.pop_state_type(State::ParsedAttrIdentifier);
        tokenizer.push_state(State::ParsedAttrColon, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_attr_separator(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ',' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::AttrSeparator);

        if tokenizer.state() == &State::ExprParsedString {
            tokenizer.pop_state_type(State::ExprParsedString);
        }

        match tokenizer.state() {
            State::ExprParsedString => tokenizer.pop_state_type(State::ExprParsedString),
            State::ExprParsedNumber => tokenizer.pop_state_type(State::ExprParsedNumber),
            _ => {}
        }

        tokenizer.pop_state_type(State::ParsedAttrColon);

        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_for_directive_value(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;
    let prev = tokenizer.tokens.last();

    todo!()
}

// Expressions

fn expect_operand_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_digit() {
        if tokenizer.state() == &State::ExprParsedOperator {
            tokenizer.pop_state_type(State::ExprParsedOperator);
        }
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ExprParsingNumber, Some(char));
        return PipelineResult::Handled;
    } else if char.is_ascii_alphabetic() || char == '_' {
        if tokenizer.state() == &State::ExprParsedOperator {
            tokenizer.pop_state_type(State::ExprParsedOperator);
        }
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ExprParsingVarOrKeyword, Some(char));
        return PipelineResult::Handled;
    } else if char == '"' {
        if tokenizer.state() == &State::ExprParsedOperator {
            tokenizer.pop_state_type(State::ExprParsedOperator);
        }
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ExprParsingString, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_string_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    // todo support single quote
    if char == '"' {
        tokenizer.pop_state_type(State::ExprExpectStringStart);
        if tokenizer.state() == &State::ExprParsedOperator {
            tokenizer.pop_state_type(State::ExprParsedOperator);
        }
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ExprParsingString, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_number_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_digit() {
        if tokenizer.state() == &State::ExprParsedNegativeOperator {
            tokenizer.pop_state_type(State::ExprParsedNegativeOperator);
        }
        if tokenizer.state() == &State::ExprParsedOperator {
            tokenizer.pop_state_type(State::ExprParsedOperator);
        }
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ExprParsingNumber, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_parenthesis_open(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '(' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::Operator(
            Operator::ParenthesesOpen,
        )));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_string(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    // todo support single quotes
    tokenizer.buf.push(char);

    if char == '"' {
        let mut trailing_backslash_count: usize = 0;
        for (i, c) in tokenizer.buf.chars().rev().enumerate() {
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
            tokenizer.push_token(TokenType::ExprToken(ExprToken::String(
                tokenizer.buf.clone(),
            )));
            tokenizer.pop_state_type(State::ExprParsingString);
            tokenizer.push_state(State::ExprParsedString, None);
        }
    }

    PipelineResult::Handled
}

fn parse_number(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if tokenizer.char.is_ascii_digit() {
        tokenizer.buf.push(char);
        return PipelineResult::Handled;
    }

    tokenizer.push_token(TokenType::ExprToken(ExprToken::Number(
        tokenizer.buf.clone(),
    )));
    tokenizer.pop_state_type(State::ExprParsingNumber);
    tokenizer.push_state(State::ExprParsedNumber, None);

    PipelineResult::Continue
}

fn parse_var_or_keyword(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_alphanumeric() || char == '_' {
        tokenizer.buf.push(char);
        return PipelineResult::Handled;
    }

    match tokenizer.buf.to_lowercase().as_str() {
        "true" => tokenizer.push_token(TokenType::ExprToken(ExprToken::True)),
        "false" => tokenizer.push_token(TokenType::ExprToken(ExprToken::False)),
        "null" => tokenizer.push_token(TokenType::ExprToken(ExprToken::Null)),
        "nan" => unimplemented!(),
        _ => {
            tokenizer.push_token(TokenType::ExprToken(
                match tokenizer.tokens.last().unwrap().r#type() {
                    TokenType::ExprToken(ExprToken::PropAccessorDot) => {
                        ExprToken::PropAccessorVar(tokenizer.buf.clone())
                    }
                    _ => ExprToken::Var(tokenizer.buf.clone()),
                },
            ));
        }
    }

    tokenizer.pop_state_type(State::ExprParsingVarOrKeyword);
    tokenizer.push_state(State::ExprParsedVarOrKeyword, None);

    PipelineResult::Continue
}

fn expect_bracket_accessor_open(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '[' {
        // todo check previous token to see what type of bracket this is.
        // todo array declaration? array access?
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::BracketAccessorOpen));
        tokenizer.push_state(State::ExprParsedBracketOpen, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_bracket_accessor_close(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ']' {
        // todo check most recent bracket type to see what type of bracket this is.
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::BracketAccessorClose));
        tokenizer.pop_state_type(State::ExprParsedBracketOpen);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_negative_operator(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '-' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::Operator(Operator::Minus)));
        tokenizer.push_state(State::ExprParsedNegativeOperator, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_operator_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if Operator::is_operator_char(char) {
        tokenizer.reset_buffer();
        tokenizer.buf.push(char);

        return match Operator::get_match(&tokenizer.buf) {
            OperatorMatchResult::Matched(op) => {
                if tokenizer.state() == &State::ExprParsedNumber {
                    tokenizer.pop_state_type(State::ExprParsedNumber);
                }
                tokenizer.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                tokenizer.push_state(State::ExprParsedOperator, None);
                PipelineResult::Handled
            }
            OperatorMatchResult::Matches(_) => {
                if tokenizer.state() == &State::ExprParsedNumber {
                    tokenizer.pop_state_type(State::ExprParsedNumber);
                }
                tokenizer.push_state(State::ExprParsingOperator, None);
                PipelineResult::Handled
            }
            OperatorMatchResult::None => PipelineResult::Err(format!(
                r#""{}" does not match any operators or is not implemented"#,
                tokenizer.buf
            )),
        };
    }

    PipelineResult::Continue
}

fn parse_operator(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if Operator::is_operator_char(char) {
        tokenizer.buf.push(char);

        match Operator::get_match(&tokenizer.buf) {
            OperatorMatchResult::Matched(op) => {
                if tokenizer.state() == &State::ExprParsedNumber {
                    tokenizer.pop_state_type(State::ExprParsedNumber);
                }
                tokenizer.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                tokenizer.push_state(State::ExprParsedOperator, None);
                tokenizer.pop_state_type(State::ExprParsingOperator);
                PipelineResult::Handled
            }
            OperatorMatchResult::Matches(_) => PipelineResult::Handled,
            OperatorMatchResult::None => PipelineResult::Err(format!(
                r#""{}" does not match any operators or is not implemented"#,
                tokenizer.buf
            )),
        }
    } else {
        match Operator::get_exact(&tokenizer.buf) {
            Some(op) => {
                tokenizer.push_token(TokenType::ExprToken(ExprToken::Operator(op)));
                tokenizer.pop_state_type(State::ExprParsingOperator);
                tokenizer.push_state(State::ExprParsedOperator, None);
                PipelineResult::Continue
            }
            None => PipelineResult::Err(format!(
                r#""{}" does not match any operators or is not implemented"#,
                tokenizer.buf
            )),
        }
    }
}

fn expect_prop_accessor(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '.' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::PropAccessorDot));
        tokenizer.pop_state_type(State::ExprParsedVarOrKeyword);
        tokenizer.push_state(State::ExprParsingVarOrKeyword, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_parenthesis_close(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == ')' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::Operator(
            Operator::ParenthesesClose,
        )));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_slash(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    PipelineResult::Continue
}

fn expect_dot(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '.' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::UnresolvedDot));
        tokenizer.push_state(State::ExprResolvingDot, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn expect_bracket_open(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    PipelineResult::Continue
}

fn resolve_dot(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;
    assert_matches!(
        tokenizer.tokens.last().unwrap().r#type(),
        TokenType::ExprToken(ExprToken::UnresolvedDot)
    );

    if char.is_ascii_digit() || matches!(char, ',' | '}' | ')') {
        tokenizer.tokens.pop(); // Pop the unresolved dot
        let prev = tokenizer.tokens.pop().unwrap();
        if let TokenType::ExprToken(ExprToken::Number(num)) = prev.r#type() {
            let mut num = num.clone();
            num.push('.');
            tokenizer.tokens.push(Token::new(
                prev.start(),
                TokenType::ExprToken(ExprToken::Number(num)),
            ));
            tokenizer.pop_state_type(State::ExprResolvingDot);
            return if char.is_ascii_digit() {
                tokenizer.push_state(State::ExprParsingFractionalNumber, Some(char));
                PipelineResult::Handled
            } else {
                PipelineResult::Continue
            };
        };
    }

    if char.is_ascii_alphabetic() {
        // Is a prop accessor dot
        tokenizer.tokens.pop(); // Pop the unresolved dot
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::PropAccessorDot));
        tokenizer.pop_state_type(State::ExprResolvingDot);
        tokenizer.push_state(State::ExprParsingVarOrKeyword, Some(char))
    }

    PipelineResult::Continue
}

fn resolve_slash(tokenizer: &mut Tokenizer) -> PipelineResult {
    todo!();
    let char = tokenizer.char;

    PipelineResult::Continue
}

fn expect_fractional_number_start(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_digit() {
        tokenizer.reset_buffer();
        tokenizer.push_state(State::ExprParsingFractionalNumber, Some(char));
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}

fn parse_fractional_number(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char.is_ascii_digit() {
        tokenizer.buf.push(char);
        return PipelineResult::Handled;
    }

    let num_tok = tokenizer.tokens.pop().unwrap();
    if let TokenType::ExprToken(ExprToken::Number(num)) = num_tok.r#type() {
        tokenizer.tokens.push(Token::new(
            num_tok.start(),
            TokenType::ExprToken(ExprToken::Number(format!("{}{}", num, tokenizer.buf))),
        ));
        tokenizer.pop_state_type(State::ExprParsingFractionalNumber);
    }

    PipelineResult::Continue
}

fn expect_concatenation_operator(tokenizer: &mut Tokenizer) -> PipelineResult {
    let char = tokenizer.char;

    if char == '+' {
        tokenizer.reset_buffer();
        tokenizer.push_token(TokenType::ExprToken(ExprToken::Operator(Operator::Plus)));
        tokenizer.pop_state_type(State::ExprParsedString);
        tokenizer.push_state(State::ExprParsedOperator, None);
        return PipelineResult::Handled;
    }

    PipelineResult::Continue
}
