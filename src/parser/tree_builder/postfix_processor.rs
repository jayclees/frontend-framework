use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::value_as_string;
use serde_json::{Number, Value};

/// Read:
/// - https://runestone.academy/ns/books/published/pythonds/BasicDS/InfixPrefixandPostfixExpressions.html
/// - https://www.geeksforgeeks.org/dsa/convert-infix-expression-to-postfix-expression/
#[derive(Debug)]
pub struct PostfixProcessor<'token> {
    infix: Vec<&'token ExprToken>,
    postfix: Vec<Part<'token>>,
    op_stack: Vec<&'token ExprToken>,
    part_builders: Vec<Part<'token>>,
    converted: bool,
}

impl<'token> PostfixProcessor<'token> {
    pub fn new() -> PostfixProcessor<'token> {
        PostfixProcessor {
            infix: vec![],
            postfix: vec![],
            op_stack: vec![],
            part_builders: vec![],
            converted: false,
        }
    }

    pub fn add(&mut self, token: &'token ExprToken) {
        self.infix.push(token);
    }

    pub fn convert(&mut self) -> &mut PostfixProcessor<'token> {
        assert!(!self.infix.is_empty(), "Infix list is empty.");

        if self.converted {
            return self;
        }

        for (i, token) in self.infix.iter().enumerate() {
            match token {
                // If the token is an operand, append it to the end of the output list.
                ExprToken::Number(_) | ExprToken::String(_) => {
                    self.postfix.push(Part::Operand(token))
                }
                ExprToken::Var(_) | ExprToken::PropAccessorVar(_) => {
                    let last = self.postfix.last_mut();
                    if let Some(Part::VarOperand(_)) = last {
                        last.unwrap().append_var_part(token);
                    } else {
                        self.postfix.push(Part::VarOperand(vec![token]))
                    }
                }
                ExprToken::PropAccessorDot => {
                    let last = self.postfix.last_mut();
                    if let Some(Part::VarOperand(var)) = last {
                        last.unwrap().append_var_part(token);
                    } else {
                        dbg!(&self.postfix);
                        panic!("Prop accessor dot has no left hand var.")
                    }
                }
                ExprToken::True | ExprToken::False => self.postfix.push(Part::Operand(token)),
                ExprToken::Operator(op) => match op {
                    // If the token is a left parenthesis, push it on the opstack.
                    Operator::ParenthesesOpen => self.op_stack.push(token),
                    // If the token is a right parenthesis, pop the opstack until
                    // the corresponding left parenthesis is removed. Append
                    // each operator to the end of the output list.
                    Operator::ParenthesesClose => {
                        let mut top_token = self.op_stack.pop().unwrap();
                        while !matches!(top_token, ExprToken::Operator(Operator::ParenthesesOpen))
                            && !self.op_stack.is_empty()
                        {
                            self.postfix.push(Part::Operator(top_token));
                            top_token = self.op_stack.pop().unwrap();
                        }
                    }
                    // If the token is an operator, *, /, +, or -, push it on the opstack. However,
                    // first remove any operators already on the opstack that have higher or
                    // equal precedence and append them to the output list.
                    Operator::Plus
                    | Operator::Minus
                    | Operator::Multiply
                    | Operator::Divide
                    | Operator::Power => {
                        while !self.op_stack.is_empty()
                            && let Some(ExprToken::Operator(next)) = self.op_stack.last()
                            && next.precedence() >= op.precedence()
                        {
                            let popped = self.op_stack.pop().unwrap();
                            self.postfix.push(Part::Operator(popped));
                        }
                        self.op_stack.push(token);
                    }
                    op => unimplemented!("Operator {op} not implemented."),
                },
                tok => unimplemented!("Token {tok} not implemented."),
            }

            // Save for debugging
            // if i == 10 {
            //     println!("Current token: {token}");
            //     for item in &self.postfix {
            //         let symbol = match item {
            //             Part::Operand(item) => item.symbol(),
            //             Part::Operator(item) => item.symbol(),
            //         };
            //         print!("{} ", symbol);
            //     }
            //     println!();
            //     for item in &self.op_stack {
            //         print!("{} ", item.symbol());
            //     }
            //     std::process::exit(1);
            // }
        }

        // Append remaining operators to postfix output
        for operator in self.op_stack.iter().rev() {
            self.postfix.push(Part::Operator(operator));
        }

        self.converted = true;

        self
    }

    pub fn evaluate<'state>(&self, states: Vec<&'state Value>) -> ValueType<'state> {
        assert!(self.converted, "Infix not converted.");

        // Save for debugging
        // for part in &self.postfix {
        //     match part {
        //         Part::Operand(tok) => print!("{} ", tok.symbol()),
        //         Part::Operator(tok) => print!("{} ", tok.symbol()),
        //     }
        // }
        // println!();

        let no_operations = self
            .postfix
            .iter()
            .filter(|p| matches!(p, Part::Operator(_)))
            .collect::<Vec<&Part>>()
            .is_empty();
        if no_operations && let Some(Part::VarOperand(query)) = self.postfix.first() {
            return ValueType::Reference(
                resolve_var_value(query, states).expect("todo handle not found"),
            );
        }

        let mut buffer: Vec<Value> = vec![];

        for part in &self.postfix {
            match part {
                Part::Operand(ExprToken::String(str)) => {
                    // Push operand onto operation stack
                    let mut str = str.clone();
                    str.remove(0);
                    str.pop();
                    buffer.push(Value::String(str));
                }
                Part::Operand(ExprToken::Number(num_str)) => {
                    // Push operand onto operation stack
                    buffer.push(Value::Number(if num_str.contains('.') {
                        Number::from_f64(num_str.parse().unwrap()).unwrap()
                    } else {
                        Number::from_i128(num_str.parse().unwrap()).unwrap()
                    }));
                }
                Part::Operand(ExprToken::True) => buffer.push(Value::Bool(true)),
                Part::Operand(ExprToken::False) => buffer.push(Value::Bool(false)),
                Part::Operator(ExprToken::Operator(operator)) => {
                    // todo handle unary operators
                    // Pop last 2 operands from operation_stack and perform operation on them.
                    let right = buffer.pop().expect("Operand buffer is empty.");
                    let left = buffer.pop().expect("Only one operand left in buffer.");

                    match operator {
                        Operator::Plus => {
                            // todo should we separate Operator::Plus into Operator::Concat?
                            // todo potentially to have different operator precedence levels.
                            if should_concat(&left, &right) {
                                buffer.push(Value::String(format!(
                                    "{}{}",
                                    value_as_string(left),
                                    value_as_string(right)
                                )));
                            } else {
                                buffer.push(add(left, right))
                            }
                        }
                        Operator::Minus => buffer.push(subtract(left, right)),
                        Operator::Multiply => buffer.push(multiply(left, right)),
                        Operator::Divide => buffer.push(divide(left, right)),
                        Operator::Power => buffer.push(power(left, right)),
                        Operator::Modulus => buffer.push(modulus(left, right)),
                        operator => unimplemented!("Operator ({operator}) not implemented."),
                    }
                }
                Part::VarOperand(query) => {
                    buffer.push(
                        resolve_var_value(query, states.to_owned())
                            .expect("todo handle not found")
                            .to_owned(),
                    );
                }
                operand => {
                    dbg!(&operand);
                    panic!("Operand not implemented.")
                }
            }
        }

        if buffer.len() != 1 {
            dbg!(buffer);
            panic!("Buf should have length of one at this point.")
        }

        ValueType::New(buffer.remove(0))
    }
}

#[derive(Debug, Clone)]
enum Part<'token> {
    Operand(&'token ExprToken),
    VarOperand(Vec<&'token ExprToken>),
    Operator(&'token ExprToken),
}

impl<'token> Part<'token> {
    pub(crate) fn append_var_part(&mut self, token: &'token ExprToken) {
        match self {
            Part::VarOperand(vec) => match token {
                ExprToken::Var(_) => {
                    if vec.is_empty() {
                        vec.push(token);
                    } else {
                        panic!("Cannot add ExprToken::Var to non empty vec here.")
                    }
                }
                ExprToken::PropAccessorDot => match vec.last() {
                    Some(ExprToken::Var(_) | ExprToken::PropAccessorVar(_)) => {
                        vec.push(token);
                    }
                    _ => {
                        dbg!(&vec);
                        panic!("Cannot push {token} to vec.")
                    }
                },
                ExprToken::PropAccessorVar(_) => {
                    if let Some(ExprToken::PropAccessorDot) = vec.last() {
                        vec.push(token);
                    } else {
                        dbg!(&vec);
                        panic!("Cannot push {token} to vec.")
                    }
                }
                _ => {
                    dbg!(&vec, token);
                    panic!("Invalid state.")
                }
            },
            Part::Operand(_) => {}
            Part::Operator(_) => {}
        }
    }
}
// enum Part<'token> {
//     Number(&'token ExprToken),
//     String(&'token ExprToken),
//     Var(&'token ExprToken),
//     FnCall(&'token ExprToken),
//     Operator(&'token ExprToken),
// }

impl<'token> Part<'token> {
    // fn get_operand(&self) -> &'token ExprToken {
    //     match self {
    //         Part::Operand(token) => token,
    //         Part::Operator(token) => panic!(r#""{token}" is not an operand."#),
    //     }
    // }
    //
    // fn get_operator(&self) -> &'token ExprToken {
    //     match self {
    //         Part::Operator(token) => token,
    //         Part::Operand(token) => panic!(r#""{token}" is not an operator."#),
    //     }
    // }
}

#[derive(Debug)]
pub(crate) enum ValueType<'state> {
    Reference(&'state Value),
    New(Value),
}

fn add(left: Value, right: Value) -> Value {
    assert!(left.is_number());
    assert!(right.is_number());

    if left.is_f64() || right.is_f64() {
        return Value::Number(
            Number::from_f64(left.as_f64().unwrap() + right.as_f64().unwrap()).unwrap(),
        );
    }

    Value::Number(
        Number::from_i128(
            left.as_number().unwrap().as_i128().unwrap()
                + right.as_number().unwrap().as_i128().unwrap(),
        )
            .unwrap(),
    )
}

fn subtract(left: Value, right: Value) -> Value {
    assert!(left.is_number());
    assert!(right.is_number());

    if left.is_f64() || right.is_f64() {
        return Value::Number(
            Number::from_f64(left.as_f64().unwrap() - right.as_f64().unwrap()).unwrap(),
        );
    }

    Value::Number(
        Number::from_i128(
            left.as_number().unwrap().as_i128().unwrap()
                - right.as_number().unwrap().as_i128().unwrap(),
        )
            .unwrap(),
    )
}

fn multiply(left: Value, right: Value) -> Value {
    assert!(left.is_number());
    assert!(right.is_number());

    if left.is_f64() || right.is_f64() {
        return Value::Number(
            Number::from_f64(left.as_f64().unwrap() * right.as_f64().unwrap()).unwrap(),
        );
    }

    Value::Number(
        Number::from_i128(
            left.as_number().unwrap().as_i128().unwrap()
                * right.as_number().unwrap().as_i128().unwrap(),
        )
            .unwrap(),
    )
}

fn divide(left: Value, right: Value) -> Value {
    assert!(left.is_number());
    assert!(right.is_number());

    if left.is_f64() || right.is_f64() {
        return Value::Number(
            Number::from_f64(left.as_f64().unwrap() / right.as_f64().unwrap()).unwrap(),
        );
    }

    Value::Number(
        Number::from_i128(
            left.as_number().unwrap().as_i128().unwrap()
                / right.as_number().unwrap().as_i128().unwrap(),
        )
            .unwrap(),
    )
}

fn power(left: Value, right: Value) -> Value {
    assert!(left.is_number());
    assert!(right.is_number());

    if left.is_f64() || right.is_f64() {
        let left = left.as_f64().unwrap();
        let right = right.as_f64().unwrap();
        return Value::Number(Number::from_f64(left.powf(right)).unwrap());
    }

    let left = left.as_number().unwrap().as_i128().unwrap();
    let right = right.as_number().unwrap().as_i128().unwrap();

    if right > u32::MAX as i128 {
        panic!("Exponent too high.")
    }

    Value::Number(Number::from_i128(left.pow(right as u32)).unwrap())
}

fn modulus(left: Value, right: Value) -> Value {
    assert!(left.is_number());
    assert!(right.is_number());

    if left.is_f64() || right.is_f64() {
        return Value::Number(
            Number::from_f64(left.as_f64().unwrap() % right.as_f64().unwrap()).unwrap(),
        );
    }

    Value::Number(
        Number::from_i128(
            left.as_number().unwrap().as_i128().unwrap()
                % right.as_number().unwrap().as_i128().unwrap(),
        )
            .unwrap(),
    )
}

fn should_concat(left: &Value, right: &Value) -> bool {
    matches!(left, Value::String(_)) || matches!(right, Value::String(_))
}

fn resolve_var_value<'state>(
    query: &Vec<&ExprToken>,
    states: Vec<&'state Value>,
) -> Option<&'state Value> {
    assert!(!query.is_empty(), "Var accessor query is empty.");

    for state in states.iter().rev() {
        // Query this state, if any part of query returns
        // None, break loop and go to the next state
        let mut x = *state;
        for (i, part) in query.iter().enumerate() {
            match part {
                ExprToken::Var(str) | ExprToken::PropAccessorVar(str) => match x.get(str) {
                    Some(result) => x = result,
                    None => break,
                },
                ExprToken::PropAccessorDot => { /* Do nothing */ }
                tok => {
                    dbg!(tok);
                    unimplemented!()
                }
            }

            if i == query.len() - 1 {
                return Some(x);
            }
        }
    }
    None
}
