// calculator.rs
use crate::types::{Operator, Token};

#[derive(Default)]
pub struct Calculator {
    pub token_stream: Vec<Token>,
    num: i64,
    number_was_pressed: bool,
    parentheses_opened: bool,
}


impl Calculator {
    pub fn new() -> Self {
        Calculator {
            token_stream: Vec::new(),
            num: 0,
            number_was_pressed: false,
            parentheses_opened: false,
        }
    }

    pub fn add_token(&mut self, token: Token) {
        match token {
            Token::Number(n) => {
                self.num = self.num * 10 + n;
                self.number_was_pressed = true;
            }
            Token::Operator(o) => {
                if !self.number_was_pressed {
                    if let Some(last) = self.token_stream.last_mut() {
                        if let Token::Operator(op) = last {
                            *op = o;
                        } else {
                            self.token_stream.push(Token::Operator(o));
                        }
                    } else {
                        self.token_stream.push(Token::Operator(o));
                    }
                } else {
                    self.token_stream.push(Token::Number(self.num));
                    self.num = 0;
                    self.token_stream.push(Token::Operator(o));
                    self.number_was_pressed = false;
                }
            }
            Token::LeftParenthesis => {
                if self.number_was_pressed {
                    self.token_stream.push(Token::Number(self.num));
                    self.num = 0;
                    self.number_was_pressed = false;
                    self.token_stream.push(Token::Operator(Operator::Multiplication));
                }
                self.token_stream.push(Token::LeftParenthesis);
                self.parentheses_opened = true;
            }
            Token::RightParenthesis => {
                if self.number_was_pressed {
                    self.token_stream.push(Token::Number(self.num));
                    self.num = 0;
                    self.number_was_pressed = false;
                }
                self.token_stream.push(Token::RightParenthesis);
            }
            Token::ClearScreen => {
                self.token_stream.clear();
                self.num = 0;
                self.number_was_pressed = false;
                self.parentheses_opened = false;
            }
            Token::ClearToken => {
                if self.number_was_pressed {
                    self.num /= 10;
                    if self.num == 0 {
                        self.number_was_pressed = false;
                    }
                } else if !self.token_stream.is_empty() {
                    self.token_stream.pop();
                }
            }
            Token::Equals => {
                if self.number_was_pressed {
                    self.token_stream.push(Token::Number(self.num));
                    self.num = 0;
                    self.number_was_pressed = false;
                }
                // Evaluation handled separately
            }
        }
    }

    pub fn get_display(&self) -> String {
        if self.token_stream.is_empty() && !self.number_was_pressed {
            return String::new();
        }
        let mut display = String::new();
        for token in &self.token_stream {
            display.push_str(&token.to_string());
            display.push(' ');
        }
        if self.number_was_pressed {
            display.push_str(&self.num.to_string());
        }
        display.trim().to_string()
    }

    fn calculate_equation(&self, operators: &[Operator], numbers: &[i64]) -> i64 {
        if numbers.is_empty() {
            return 0;
        }

        let mut ops = operators.to_vec();
        let mut nums = numbers.to_vec();

        let mut i = 0;
        while i < ops.len() {
            match ops[i] {
                Operator::Multiplication => {
                    let result = nums[i] * nums[i + 1];
                    nums[i] = result;
                    nums.remove(i + 1);
                    ops.remove(i);
                }
                Operator::Division => {
                    let result = nums[i] / nums[i + 1];
                    nums[i] = result;
                    nums.remove(i + 1);
                    ops.remove(i);
                }
                _ => i += 1,
            }
        }

        if ops.is_empty() || nums.len() == 1 {
            return nums[0];
        }

        let mut result = nums[0];
        for (i, op) in ops.iter().enumerate() {
            let next_num = nums[i + 1];
            match op {
                Operator::Addition => result += next_num,
                Operator::Subtraction => result -= next_num,
                _ => {}
            }
        }
        result
    }

    fn evaluate_full_expression(&mut self) -> i64 {
        while self.token_stream.len() > 1 {
            let mut numbers = Vec::new();
            let mut operators = Vec::new();
            for token in self.token_stream.drain(..) {
                match token {
                    Token::Number(n) => numbers.push(n),
                    Token::Operator(o) => operators.push(o),
                    _ => {}
                }
            }
            let result = self.calculate_equation(&operators, &numbers);
            self.token_stream.push(Token::Number(result));
        }
        if let Some(Token::Number(result)) = self.token_stream.first() {
            *result
        } else {
            0
        }
    }

    pub fn evaluate(&mut self) -> i64 {
        if !self.token_stream.iter().any(|t| matches!(t, Token::LeftParenthesis)) {
            return self.evaluate_full_expression();
        }

        while self.token_stream.iter().any(|t| matches!(t, Token::LeftParenthesis)) {
            let mut stack = Vec::new();
            let mut start_idx = None;
            let mut end_idx = None;

            for (i, token) in self.token_stream.iter().enumerate() {
                match token {
                    Token::LeftParenthesis => stack.push(i),
                    Token::RightParenthesis => {
                        if let Some(idx) = stack.pop() {
                            start_idx = Some(idx);
                            end_idx = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let start_idx = start_idx.expect("Mismatched parentheses");
            let end_idx = end_idx.expect("Mismatched parentheses");

            let mut numbers = Vec::new();
            let mut operators = Vec::new();
            for token in self.token_stream.drain(start_idx..=end_idx) {
                match token {
                    Token::Number(n) => numbers.push(n),
                    Token::Operator(o) => operators.push(o),
                    _ => {}
                }
            }

            let result = self.calculate_equation(&operators, &numbers);
            self.token_stream.insert(start_idx, Token::Number(result));
            println!("\nEvaluated parentheses -> {:?}", self.token_stream);
        }

        self.evaluate_full_expression()
    }
}