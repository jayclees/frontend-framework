use crate::parser::tokenizer::token::{Expr, Operator, TokenType};

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

    assert_eq!(13, TokenType::ExprToken(Expr::String(r#""hello world""#.to_owned())).len());
    assert_eq!(13, TokenType::ExprToken(Expr::String(r#""hello world""#.to_owned())).len());
    assert_eq!(20, TokenType::ExprToken(Expr::String(r#""this is a sentence""#.to_owned())).len());
    assert_eq!(3, TokenType::ExprToken(Expr::String(r#""a""#.to_owned())).len());

    //

    assert_eq!(1, TokenType::ExprToken(Expr::Number("1".to_owned())).len());
    assert_eq!(3, TokenType::ExprToken(Expr::Number("123".to_owned())).len());
    assert_eq!(5, TokenType::ExprToken(Expr::Number("12345".to_owned())).len());

    //

    assert_eq!(1, TokenType::ExprToken(Expr::Var("x".to_owned())).len());
    assert_eq!(6, TokenType::ExprToken(Expr::Var("my_var".to_owned())).len());
    assert_eq!(8, TokenType::ExprToken(Expr::Var("my_var_2".to_owned())).len());
    assert_eq!(11, TokenType::ExprToken(Expr::Var("another_var".to_owned())).len());

    //

    assert_eq!(7, TokenType::ExprToken(Expr::Var("my_func".to_owned())).len());
    assert_eq!(13, TokenType::ExprToken(Expr::Var("my_other_func".to_owned())).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Var("a".to_owned())).len());
}

#[test]
fn test_token_len_static() {
    assert_eq!(1, TokenType::BlockOpen.len());
    assert_eq!(1, TokenType::BlockClose.len());
    assert_eq!(1, TokenType::AttrColon.len());
    assert_eq!(1, TokenType::AttrSeparator.len());
    assert_eq!(1, TokenType::DirectiveOpen.len());
    assert_eq!(1, TokenType::DirectiveClose.len());
    assert_eq!(1, TokenType::DirectiveColon.len());
    assert_eq!(2, TokenType::EventListenerOpen.len()); // event listener open
    assert_eq!(1, TokenType::EventListenerClose.len());
    assert_eq!(1, TokenType::EventListenerColon.len());
    assert_eq!(1, TokenType::ExprToken(Expr::VarDotAccessor).len());
    assert_eq!(1, TokenType::ExprToken(Expr::VarBracketAccessorOpen).len());
    assert_eq!(1, TokenType::ExprToken(Expr::VarBracketAccessorClose).len());
    assert_eq!(1, TokenType::ExprToken(Expr::FunctionArgsOpen).len());
    assert_eq!(1, TokenType::ExprToken(Expr::FunctionArgsClose).len());
    assert_eq!(1, TokenType::ExprToken(Expr::ParenthesesOpen).len());
    assert_eq!(1, TokenType::ExprToken(Expr::ParenthesesClose).len());
    assert_eq!(1, TokenType::ExprToken(Expr::ArrayOpen).len());
    assert_eq!(1, TokenType::ExprToken(Expr::ArrayClose).len());
}

#[test]
fn test_symbol_len() {
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Plus)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Minus)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Multiply)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Power)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Divide)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Modulus)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Assign)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::Increment)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::Decrement)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::IncrementBy)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::DecrementBy)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::MultiplyBy)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::DivideBy)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::EqualTo)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::NotEqualTo)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::GreaterThan)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::GreaterThanOrEqualTo)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::LessThan)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::LessThanOrEqualTo)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::And)).len());
    assert_eq!(2, TokenType::ExprToken(Expr::Operator(Operator::Or)).len());
    assert_eq!(1, TokenType::ExprToken(Expr::Operator(Operator::Not)).len());
}
