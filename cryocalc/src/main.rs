use iced::widget::{button, column, row, text, text_input};
use iced::Element;
use std::fmt;

// This is the struct that will eventually be able to parse the expressions
// Through something like  ```let result = ExpressionParser::Parse("1 + 5 / (9 * 3)");```
struct ExpressionParser {}

#[derive(Default)]
enum CalculatorMode {
    #[default]
    Regular,
    Hexadecimal,
    Binary,
}

// This struct, holds the state of the entire application
#[derive(Default)]
struct CryoCalc {
    // Holds the state for our display
    display_content: String,

    // Used to decide what to display
    mode: CalculatorMode,

    // Used to store the tokens
    token_stream: Vec<Token>,

    // Current user given number and power for calculating the value right away
    num: i64,

    parentheses_opened: bool,
    number_was_pressed: bool,
    number_of_tokens: usize,
    //left_parentheses_count: usize,
    //parentheses_maxdepth: usize,
}

#[derive(Clone)]
enum Operator {
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
enum Token {
    Number(i64),
    LeftParenthesis,
    RightParenthesis,
    Operator(Operator),
    Equals
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Token::Number(n) => n.to_string(),
            Token::LeftParenthesis => "(".to_string(),
            Token::RightParenthesis => ")".to_string(),
            Token::Operator(op) => format!("{:?}", op),
            Token::Equals => "=".to_string(),
        };
        write!(f, "{}", token_str)
    }
}

pub fn main() -> iced::Result {
    iced::run("CryoCalc", CryoCalc::update, CryoCalc::view)
}

impl CryoCalc {
    // Existing function that evaluates an expression represented by numbers and operators.
    fn calculate_equation(operators: &Vec<Operator>, numbers: &Vec<i64>) -> i64 {
        // Handle empty input case
        if numbers.is_empty() {
            return 0;
        }
        
        // Create mutable copies
        let mut ops = operators.clone();
        let mut nums = numbers.clone();
        
        // First pass: Process multiplication and division
        let mut i = 0;
        while i < ops.len() {
            match ops[i] {
                Operator::Multiplication => {
                    let result = nums[i] * nums[i + 1];
                    nums[i] = result;
                    nums.remove(i + 1);
                    ops.remove(i);
                },
                Operator::Division => {
                    let result = nums[i] / nums[i + 1];
                    nums[i] = result;
                    nums.remove(i + 1);
                    ops.remove(i);
                },
                _ => i += 1,
            }
        }
        
        // If we have only one number left (or no operators), return it directly
        if ops.is_empty() || nums.len() == 1 {
            return nums[0];
        }
        
        // Second pass: Process addition and subtraction
        let mut result = nums[0];
        for (i, op) in ops.iter().enumerate() {
            let next_num = nums[i + 1];
            match op {
                Operator::Addition => result += next_num,
                Operator::Subtraction => result -= next_num,
                _ => {} // Should not occur
            }
        }
        
        result
    }

    fn parse_tokens(&mut self) {
        // If there are no parentheses, evaluate the full expression.
        if !self.token_stream.iter().any(|t| matches!(t, Token::LeftParenthesis)) {
            self.display_content = self.evaluate_full_expression().to_string();
            return;
        }
    
        // Process innermost parentheses first.
        while self.token_stream.iter().any(|t| matches!(t, Token::LeftParenthesis)) {
            let mut stack = Vec::new();
            let mut start_idx = None;
            let mut end_idx = None;
    
            // Scan left-to-right to find first complete innermost pair.
            for (i, token) in self.token_stream.iter().enumerate() {
                match token {
                    Token::LeftParenthesis => stack.push(i),
                    Token::RightParenthesis => {
                        if let Some(idx) = stack.pop() {
                            start_idx = Some(idx);
                            end_idx = Some(i);
                            break;
                        }
                    },
                    _ => {}
                }
            }
    
            // Unwrap the found indices.
            let start_idx = start_idx.expect("Mismatched parentheses: no matching '(' found");
            let end_idx = end_idx.expect("Mismatched parentheses: no ')' found");
    
            // Extract tokens between the matched pair.
            let mut numbers: Vec<i64> = Vec::new();
            let mut operators: Vec<Operator> = Vec::new();
    
            // Drain tokens from start_idx to end_idx (inclusive)
            for token in self.token_stream.drain(start_idx..=end_idx) {
                match token {
                    Token::Number(n) => numbers.push(n),
                    Token::Operator(o) => operators.push(o),
                    _ => {} // Skip parentheses
                }
            }
    
            let result = CryoCalc::calculate_equation(&operators, &numbers);
            // Insert the result into the token stream.
            self.token_stream.insert(start_idx, Token::Number(result));
            println!("\nEvaluated parentheses -> {:?}", self.token_stream);
        }
    
        // Finally, evaluate any remaining expression.
        self.display_content = self.evaluate_full_expression().to_string();
    }

    fn evaluate_full_expression(&mut self) -> i64 {
        while self.token_stream.len() > 1 {
            let mut numbers: Vec<i64> = Vec::new();
            let mut operators: Vec<Operator> = Vec::new();
    
            // Remove ALL tokens and process them
            for token in self.token_stream.drain(..) {
                match token {
                    Token::Number(n) => numbers.push(n),
                    Token::Operator(o) => operators.push(o),
                    _ => {} // Ignore other tokens
                }
            }
    
            // Calculate result and push it back to the token stream.
            let result = CryoCalc::calculate_equation(&operators, &numbers);
            self.token_stream.push(Token::Number(result));
        }
    
        // After processing, expect exactly one token with the final result
        if let Some(Token::Number(result)) = self.token_stream.first() {
            *result
        } else {
            panic!("Expected a Number token as the final result")  // or return a default value
        }
    }

    fn update(self: &mut CryoCalc, message: Message) {
        match message {
            Message::DisplayContentChanged(content) => {
                self.display_content = content;
            },
            Message::ButtonPressed(value) => {
                match value {
                    Token::Number(n) => {
                        self.num = self.num * 10 + n;
                        self.number_was_pressed = true;
                    },
                    Token::Operator(o) => {
                        if !self.number_was_pressed {
                            // No new number pressed; update the last operator if present.
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
                            // A number was pressed. Push the number and then the operator.
                            self.token_stream.push(Token::Number(self.num));
                            self.num = 0;
                            self.token_stream.push(Token::Operator(o));
                            self.number_was_pressed = false;
                        }
                    },
                    Token::LeftParenthesis => {
                        if self.number_was_pressed {
                            self.token_stream.push(Token::Number(self.num));
                            self.num = 0;
                            self.number_was_pressed = false;

                            self.token_stream.push(Token::Operator(Operator::Multiplication));
                            self.token_stream.push(Token::LeftParenthesis);
                        } else {
                            self.token_stream.push(Token::LeftParenthesis);
                        }

                        self.parentheses_opened = true;                        
                        //self.left_parentheses_count += 1;
                    },
                    Token::RightParenthesis => {

                        // If a number was pressed, push it first.
                        if self.number_was_pressed {
                            self.token_stream.push(Token::Number(self.num));
                            self.num = 0;
                            self.number_was_pressed = false;
                            
                            self.token_stream.push(Token::RightParenthesis);
                        } else {
                            self.token_stream.push(Token::RightParenthesis);
                        }
                            //self.parentheses_maxdepth = self.parentheses_maxdepth.max(self.left_parentheses_count);
                            //self.left_parentheses_count = 0;
                    },
                    Token::Equals => {
                        if self.number_was_pressed {
                            self.token_stream.push(Token::Number(self.num));
                            self.num = 0;
                            self.number_was_pressed = false;
                        }
                        self.parse_tokens();
                    }
                }
                println!("\n{:?}", self.token_stream);
                self.number_of_tokens += 1;
            },
        }
    }

    fn view(self: &CryoCalc) -> Element<Message> {
       match self.mode {
           _ => {
                column![
                    // Example, shows a text input widget w
                    text_input("...", &self.display_content)
                        .on_input(Message::DisplayContentChanged),
                    button("0").on_press(Message::ButtonPressed(Token::Number(0))),
                    button("1").on_press(Message::ButtonPressed(Token::Number(1))),
                    button("2").on_press(Message::ButtonPressed(Token::Number(2))),
                    button("3").on_press(Message::ButtonPressed(Token::Number(3))),
                    button("4").on_press(Message::ButtonPressed(Token::Number(4))),
                    button("5").on_press(Message::ButtonPressed(Token::Number(5))),
                    button("6").on_press(Message::ButtonPressed(Token::Number(6))),
                    button("7").on_press(Message::ButtonPressed(Token::Number(7))),
                    button("8").on_press(Message::ButtonPressed(Token::Number(8))),
                    button("9").on_press(Message::ButtonPressed(Token::Number(9))),

                    button("+").on_press(Message::ButtonPressed(Token::Operator(Operator::Addition))),
                    button("-").on_press(Message::ButtonPressed(Token::Operator(Operator::Subtraction))),
                    button("*").on_press(Message::ButtonPressed(Token::Operator(Operator::Multiplication))),
                    button("/").on_press(Message::ButtonPressed(Token::Operator(Operator::Division))),
                    button("(").on_press(Message::ButtonPressed(Token::LeftParenthesis)),
                    button(")").on_press(Message::ButtonPressed(Token::RightParenthesis)),
                    button("=").on_press(Message::ButtonPressed(Token::Equals)),


                ].into()
           },
       }
    }
}

#[derive(Debug, Clone)]
enum Message {
    DisplayContentChanged(String),
    ButtonPressed(Token)
    //Likely this needs some kind of button pressed message that tells us which button has been
    //pressed?
}
