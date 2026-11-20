use crate::parser::tokenizer::token::{ExprToken, Operator};
use crate::parser::tree_builder::block::BlockContent;
use serde_json::{Number, Value};

/// Read:
/// - https://runestone.academy/ns/books/published/pythonds/BasicDS/InfixPrefixandPostfixExpressions.html
/// - https://www.geeksforgeeks.org/dsa/convert-infix-expression-to-postfix-expression/
#[derive(Debug)]
pub struct PostfixProcessor<'token> {
    op_stack: Vec<&'token ExprToken>,
    postfix_expr: Vec<Part<'token>>,
    step: usize,
    part_builders: Vec<Part<'token>>,
}

impl<'token> PostfixProcessor<'token> {
    pub fn new() -> PostfixProcessor<'token> {
        PostfixProcessor {
            op_stack: vec![],
            postfix_expr: vec![],
            step: 0,
            part_builders: vec![],
        }
    }

    pub fn process(&mut self, token: &'token ExprToken) {
        match token {
            ExprToken::Number(_) | ExprToken::String(_) => {
                self.postfix_expr.push(Part::Operand(token))
            }
            ExprToken::Operator(op) => {
                match op {
                    Operator::Plus
                    | Operator::Minus
                    | Operator::Multiply
                    | Operator::Divide
                    | Operator::Power => {
                        // Pop each operator on the stack that has higher
                        // or equal precedence and append to output.
                        for i in (0..self.op_stack.len()).rev() {
                            let to_pop_token = self.op_stack.get(i);
                            if let Some(ExprToken::Operator(to_pop_op)) = to_pop_token
                                && to_pop_op.precedence() >= op.precedence()
                            {
                                let popped = self.op_stack.remove(i);
                                self.postfix_expr.push(Part::Operator(popped));
                            }
                        }
                        self.op_stack.push(token);
                    }
                    Operator::ParenthesesOpen => self.op_stack.push(token),
                    Operator::ParenthesesClose => {
                        // Pop all operators and append to output until a left parenthesis is found.
                        for i in (0..self.op_stack.len()).rev() {
                            let to_pop_token = self.op_stack.get(i);
                            if let Some(ExprToken::Operator(Operator::ParenthesesOpen)) =
                                to_pop_token
                            {
                                self.op_stack.remove(i);
                                break;
                            } else {
                                let to_add_op = self.op_stack.remove(i);
                                self.postfix_expr.push(Part::Operator(to_add_op))
                            }
                        }
                    }
                    op => unimplemented!("Operator {op} not implemented."),
                }
            }
            tok => unimplemented!("Token {tok} not implemented."),
        }
        self.step += 1;
    }

    pub fn resolve(&mut self) -> BlockContent<'token> {
        // Append remaining operators to postfix output
        for operator in self.op_stack.iter().rev() {
            self.postfix_expr.push(Part::Operator(operator));
        }

        println!("Infix:   1 + 2 - 3 * (4 + 1) + 1");
        print!("PostFix: ");
        for part in &self.postfix_expr {
            match part {
                Part::Operand(tok) => print!("{} ", tok.symbol()),
                Part::Operator(tok) => print!("{} ", tok.symbol()),
            }
        }
        println!();

        let mut output: Vec<Part> = std::mem::take(&mut self.postfix_expr);
        let mut operation_stack: Vec<Value> = vec![];

        while !output.is_empty() {
            match output.remove(0) {
                Part::Operand(ExprToken::String(str)) => {
                    // Push operand onto operation stack
                    let mut str = str.clone();
                    str.remove(0);
                    str.pop();
                    operation_stack.push(Value::String(str));
                }
                Part::Operand(ExprToken::Number(num_str)) => {
                    // Push operand onto operation stack
                    let num = if num_str.contains('.') {
                        Number::from_f64(num_str.parse().unwrap()).unwrap()
                    } else {
                        Number::from_i128(num_str.parse().unwrap()).unwrap()
                    };
                    operation_stack.push(Value::Number(num));
                }
                Part::Operator(ExprToken::Operator(operator)) => {
                    // Pop last 2 operands from operation_stack and perform operation on them.
                    let right = operation_stack.pop().expect("Operand buffer is empty.");
                    let left = operation_stack
                        .pop()
                        .expect("Only one operand left in buffer.");

                    match operator {
                        Operator::Plus => {
                            // todo should we separate Operator::Plus into Operator::Concat?
                            // todo potentially to have different operator precedence levels.
                            let is_concat = matches!(left, Value::String(_))
                                || matches!(right, Value::String(_));
                            if is_concat {
                                let left = match left {
                                    Value::String(s) => s,
                                    Value::Number(n) => n.to_string(),
                                    Value::Null => "null".to_string(),
                                    Value::Bool(bool) => bool.to_string(),
                                    Value::Array(_) => unimplemented!(),
                                    Value::Object(_) => unimplemented!(),
                                };
                                operation_stack.push(Value::String(format!("{left}{right}")));
                            } else {
                                operation_stack.push(add(left, right))
                            }
                        }
                        Operator::Minus => operation_stack.push(subtract(left, right)),
                        Operator::Multiply => operation_stack.push(multiply(left, right)),
                        Operator::Divide => operation_stack.push(divide(left, right)),
                        Operator::Power => operation_stack.push(power(left, right)),
                        Operator::Modulus => operation_stack.push(modulus(left, right)),
                        _ => unimplemented!(),
                    }
                }
                _ => panic!("Non operator found in Part::Operator."),
            }
        }

        // dbg!(&buf);

        self.reset();

        if operation_stack.len() > 1 {
            dbg!(operation_stack);
            panic!("Buf should have length of one at this point.")
        }

        if let Some(Value::Number(num)) = operation_stack.first() {
            BlockContent::Text(num.to_string())
        } else if let Some(Value::String(str)) = operation_stack.first() {
            // let chars = str.chars();
            // let string = chars.skip(1).take(str.len() - 2).collect::<String>();
            BlockContent::Text(str.to_owned())
        } else {
            panic!("Buf should not be empty.");
        }
    }

    pub fn is_empty(&self) -> bool {
        self.postfix_expr.is_empty()
    }

    pub fn reset(&mut self) {
        self.postfix_expr = vec![];
        self.op_stack = vec![];
    }
}

#[derive(Debug, Clone)]
enum Part<'a> {
    Operand(&'a ExprToken),
    Operator(&'a ExprToken),
}
// enum Part<'token> {
//     Number(&'token ExprToken),
//     String(&'token ExprToken),
//     Var(&'token ExprToken),
//     FnCall(&'token ExprToken),
//     Operator(&'token ExprToken),
// }

impl<'token> Part<'token> {
    fn get_operand(&self) -> &'token ExprToken {
        match self {
            Part::Operand(token) => token,
            Part::Operator(token) => panic!(r#""{token}" is not an operand."#),
        }
    }

    fn get_operator(&self) -> &'token ExprToken {
        match self {
            Part::Operator(token) => token,
            Part::Operand(token) => panic!(r#""{token}" is not an operator."#),
        }
    }
}

fn add(left: Value, right: Value) -> Value {
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
