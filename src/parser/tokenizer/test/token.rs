use crate::parser::tokenizer::token::{Operator, TokenType};

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

    assert_eq!(13, TokenType::ExprString(r#""hello world""#.to_owned()).len());
    assert_eq!(20, TokenType::ExprString(r#""this is a sentence""#.to_owned()).len());
    assert_eq!(3, TokenType::ExprString(r#""a""#.to_owned()).len());

    //

    assert_eq!(1, TokenType::ExprNumber("1".to_owned()).len());
    assert_eq!(3, TokenType::ExprNumber("123".to_owned()).len());
    assert_eq!(5, TokenType::ExprNumber("12345".to_owned()).len());

    //

    assert_eq!(1, TokenType::ExprVariable("x".to_owned()).len());
    assert_eq!(6, TokenType::ExprVariable("my_var".to_owned()).len());
    assert_eq!(8, TokenType::ExprVariable("my_var_2".to_owned()).len());
    assert_eq!(11, TokenType::ExprVariable("another_var".to_owned()).len());

    //

    assert_eq!(9, TokenType::ExprFunctionCall("my_func()".to_owned()).len());
    assert_eq!(17, TokenType::ExprFunctionCall(r#"with_arg("hello")"#.to_owned()).len());
    assert_eq!(37, TokenType::ExprFunctionCall(r#"with_args("arg1", arg2, arg3.prop, 3)"#.to_owned()).len());
}

#[test]
fn test_token_len_static() {
    assert_eq!(1, TokenType::Colon.len());
    assert_eq!(1, TokenType::BlockOpen.len());
    assert_eq!(1, TokenType::BlockClose.len());
    assert_eq!(1, TokenType::AttrColon.len());
    assert_eq!(1, TokenType::AttrSeparator.len());
    assert_eq!(1, TokenType::Expression.len());
    assert_eq!(1, TokenType::DirectiveOpen.len());
    assert_eq!(1, TokenType::DirectiveClose.len());
    assert_eq!(1, TokenType::DirectiveColon.len());
    assert_eq!(2, TokenType::EventListenerOpen.len()); // event listener open
    assert_eq!(1, TokenType::EventListenerClose.len());
    assert_eq!(1, TokenType::EventListenerColon.len());
    assert_eq!(1, TokenType::ExprParenthesesOpen.len());
    assert_eq!(1, TokenType::ExprParenthesesClose.len());
}

#[test]
fn test_symbol_len() {
    assert_eq!(1, TokenType::ExprOperator(Operator::Add).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::Sub).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::Multiply).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::Power).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::Divide).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::Modulus).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::Assign).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::Increment).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::Decrement).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::IncrementBy).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::DecrementBy).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::MultiplyBy).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::DivideBy).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::EqualTo).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::NotEqualTo).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::GreaterThan).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::GreaterThanOrEqualTo).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::LessThan).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::LessThanOrEqualTo).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::And).len());
    assert_eq!(2, TokenType::ExprOperator(Operator::Or).len());
    assert_eq!(1, TokenType::ExprOperator(Operator::Not).len());
}
