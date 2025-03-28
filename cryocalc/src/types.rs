use std::fmt;

#[derive(Default, Clone)]
pub enum CalculatorMode {
    #[default]
    Regular,
    Hexadecimal,
    Binary,
}

#[derive(Clone)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            Operator::Addition => "+",
            Operator::Subtraction => "-",
            Operator::Multiplication => "*",
            Operator::Division => "/",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Clone)]
pub enum Token {
    Number(i64),
    LeftParenthesis,
    RightParenthesis,
    Operator(Operator),
    Equals,
    ClearScreen,
    ClearToken,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Token::Number(n) => n.to_string(),
            Token::LeftParenthesis => "(".to_string(),
            Token::RightParenthesis => ")".to_string(),
            Token::Operator(op) => format!("{:?}", op),
            Token::Equals => "=".to_string(),
            Token::ClearScreen => "CLEAR".to_string(),
            Token::ClearToken => "<<".to_string(),
        };
        write!(f, "{}", token_str)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
