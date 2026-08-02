pub(super) struct ExpressionTokenizer {
    state_history: Vec<State>,
    cursor: usize,
    buf: String,
    buf_start: Option<usize>,
    tokens: Vec<Token>,
}

impl ExpressionTokenizer {
    pub(super) fn new() -> ExpressionTokenizer {
        ExpressionTokenizer {
            state_history: vec![],
            cursor: 0,
            buf: "".to_string(),
            buf_start: None,
            tokens: vec![],
        }
    }

    pub(super) fn tokenize(string: &str) {
        let chars = string.chars();

        for char in chars.enumerate() {
            //
        }
    }
}

enum State {
    Start,
    ParsingString,
    ParsedString,
    ParsingNumber,
    ParsedNumber,
    ParsingFunctionCall,
    ParsedFunctionCall,
    ParsingOperator,
    ParsedOperator,
    ParsingParenthesesOpen,
    ParsedParenthesesOpen,
    ParsingParenthesesClosed,
    ParsedParenthesesClosed,
}

struct Token {
    start: usize,
    end: usize,
}

enum TokenType {
    String,
    Number,
    Variable,
    FunctionCall,
    Operator(Symbol),
    ParenthesesOpen,
    ParenthesesClose,
}

enum Symbol {
    Variable(String),

    // Arithmetic
    Add,
    Sub,
    Multiply,
    Power, // ^x
    Divide,
    Modulus,

    // Assignment operators
    Assign,  // =
    Increment,   // ++
    Decrement,   // --
    IncrementBy, // +=
    DecrementBy, // -=
    MultiplyBy,  // *=
    DivideBy,    // /=

    // Comparisons
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,

    // Logical
    And,
    Or,
    Not,
}
