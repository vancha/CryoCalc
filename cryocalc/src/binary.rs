use crate::{calculator::Calculator, types::Token, types::Operator};

use iced::{
    widget::{button, column, row, text_input, Button, Text},
    Element, Length,
};

#[derive(Default)]
pub struct BinCalcState {
    calculator: Calculator,
    display_content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Token),
    DisplayContentChanged(String),
}

impl BinCalcState {
    fn button<'a>(token: Token) -> Element<'a, Message> {
        let label = match token {
            Token::Number(0) => "0".to_string(),
            Token::Number(1) => "1".to_string(),
            _ => token.to_string(),
        };
        
        button(Text::new(label))
            .on_press(Message::ButtonPressed(token))
            .width(Length::Fill)
            .padding(16)
            .into()
    }

    pub fn new() -> Self {
        BinCalcState {
            calculator: Calculator::new().with_base(2),
            display_content: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed(token) => {
                if token.is_valid_for_base(2) {
                    self.calculator.add_token(token.clone());
                    if let Token::Equals = token {
                        let result = self.calculator.evaluate();
                        self.display_content = format!("{:b}", result);
                    } else {
                        self.display_content = self.calculator.get_display();
                    }
                }
            }
            Message::DisplayContentChanged(_) => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let space = 5;
        column![
            column![
                text_input("...", &self.display_content)
                    .padding(space)
                    .on_input(Message::DisplayContentChanged),
            ]
            .padding(15),
            column![
                row![
                    BinCalcState::button(Token::ClearScreen),
                    BinCalcState::button(Token::LeftParenthesis),
                    BinCalcState::button(Token::RightParenthesis),
                    BinCalcState::button(Token::Operator(Operator::Division)),
                    BinCalcState::button(Token::ClearToken),
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    BinCalcState::button(Token::Number(0)),
                    BinCalcState::button(Token::Number(1)),
                    BinCalcState::button(Token::Operator(Operator::Multiplication)),
                    BinCalcState::button(Token::Operator(Operator::Subtraction))
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    BinCalcState::button(Token::Operator(Operator::Addition)),
                    BinCalcState::button(Token::Equals)
                ]
                .spacing(space * 2)
                .padding(space),
            ]
            .padding(10)
        ]
        .into()
    }
}