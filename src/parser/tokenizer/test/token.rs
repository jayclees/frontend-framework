use crate::parser::tokenizer::token::{ExprToken, Operator, TokenType};

#[test]
#[rustfmt::skip]
fn test_token_len_with_val() {
    assert_eq!(11, TokenType::BlockIdentifier("MyComponent".to_owned()).len());
    assert_eq!(4, TokenType::BlockIdentifier("Page".to_owned()).len());
    assert_eq!(1, TokenType::BlockIdentifier("p".to_owned()).len());

    //

    assert_eq!(8, TokenType::LineComment("// hello".to_owned()).len());
    assert_eq!(2, TokenType::LineComment("//".to_owned()).len());
    assert_eq!(3, TokenType::LineComment("//1".to_owned()).len());

    //

    assert_eq!(17, TokenType::BlockComment("/* hello world */".to_owned()).len());
    assert_eq!(6, TokenType::BlockComment("/*  */".to_owned()).len());
    assert_eq!(4, TokenType::BlockComment("/**/".to_owned()).len());
    assert_eq!(5, TokenType::BlockComment("/* */".to_owned()).len());
    assert_eq!(18, TokenType::BlockComment("/*\nhello world!\n*/".to_owned()).len());

    //

    assert_eq!(5, TokenType::AttrIdentifier("title".to_owned()).len());
    assert_eq!(2, TokenType::AttrIdentifier("id".to_owned()).len());
    assert_eq!(11, TokenType::AttrIdentifier("data-custom".to_owned()).len());

    //

    assert_eq!(2, TokenType::DirectiveIdentifier("if".to_owned()).len());
    assert_eq!(3, TokenType::DirectiveIdentifier("for".to_owned()).len());
    assert_eq!(16, TokenType::DirectiveIdentifier("marker-directive".to_owned()).len());

    //

    assert_eq!(9, TokenType::DirectiveValue("something".to_owned()).len());
    assert_eq!(13, TokenType::DirectiveValue("post in posts".to_owned()).len());

    //

    assert_eq!(5, TokenType::EventListenerIdentifier("click".to_owned()).len());
    assert_eq!(4, TokenType::EventListenerIdentifier("load".to_owned()).len());
    assert_eq!(12, TokenType::EventListenerIdentifier("custom-event".to_owned()).len());

    //

    assert_eq!(10, TokenType::EventListenerHandler("do_thing()".to_owned()).len());
    assert_eq!(9, TokenType::EventListenerHandler("counter++".to_owned()).len());
    assert_eq!(13, TokenType::EventListenerHandler("something = 1".to_owned()).len());

    //

    assert_eq!(13, TokenType::ExprToken(ExprToken::String(r#""hello world""#.to_owned())).len());
    assert_eq!(13, TokenType::ExprToken(ExprToken::String(r#""hello world""#.to_owned())).len());
    assert_eq!(20, TokenType::ExprToken(ExprToken::String(r#""this is a sentence""#.to_owned())).len());
    assert_eq!(3, TokenType::ExprToken(ExprToken::String(r#""a""#.to_owned())).len());

    //

    assert_eq!(1, TokenType::ExprToken(ExprToken::Number("1".to_owned())).len());
    assert_eq!(3, TokenType::ExprToken(ExprToken::Number("123".to_owned())).len());
    assert_eq!(5, TokenType::ExprToken(ExprToken::Number("12345".to_owned())).len());

    //

    assert_eq!(1, TokenType::ExprToken(ExprToken::Var("x".to_owned())).len());
    assert_eq!(6, TokenType::ExprToken(ExprToken::Var("my_var".to_owned())).len());
    assert_eq!(8, TokenType::ExprToken(ExprToken::Var("my_var_2".to_owned())).len());
    assert_eq!(11, TokenType::ExprToken(ExprToken::Var("another_var".to_owned())).len());

    //

    assert_eq!(7, TokenType::ExprToken(ExprToken::Var("my_func".to_owned())).len());
    assert_eq!(13, TokenType::ExprToken(ExprToken::Var("my_other_func".to_owned())).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Var("a".to_owned())).len());
}

#[test]
fn test_token_len_static() {
    assert_eq!(1, TokenType::BlockOpen.len());
    assert_eq!(1, TokenType::BlockClose.len());
    assert_eq!(1, TokenType::AttrColon.len());
    assert_eq!(1, TokenType::BlockOrAttrSeparator.len());
    assert_eq!(1, TokenType::DirectiveOpen.len());
    assert_eq!(1, TokenType::DirectiveClose.len());
    assert_eq!(1, TokenType::DirectiveColon.len());
    assert_eq!(2, TokenType::EventListenerOpen.len()); // event listener open
    assert_eq!(1, TokenType::EventListenerClose.len());
    assert_eq!(1, TokenType::EventListenerColon.len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::VarDotAccessor).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::VarBracketAccessorOpen).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::VarBracketAccessorClose).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::FunctionArgsOpen).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::FunctionArgsClose).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::ParenthesesOpen).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::ParenthesesClose).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::ArrayOpen).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::ArrayClose).len());
}

#[test]
fn test_symbol_len() {
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Plus)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Minus)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Multiply)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Power)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Divide)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Modulus)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Assign)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::Increment)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::Decrement)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::IncrementBy)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::DecrementBy)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::MultiplyBy)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::DivideBy)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::EqualTo)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::NotEqualTo)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::GreaterThan)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::GreaterThanOrEqualTo)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::LessThan)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::LessThanOrEqualTo)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::And)).len());
    assert_eq!(2, TokenType::ExprToken(ExprToken::Operator(Operator::Or)).len());
    assert_eq!(1, TokenType::ExprToken(ExprToken::Operator(Operator::Not)).len());
}
