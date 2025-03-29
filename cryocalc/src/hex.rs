use crate::{calculator::Calculator, types::Token, types::Operator};

use iced::{
    widget::{button, column, row, text_input, Button, Text},
    Element, Length,
};

#[derive(Default)]
pub struct HexCalcState {
    calculator: Calculator,
    display_content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Token),
    DisplayContentChanged(String),
}

impl HexCalcState {
    fn button<'a>(token: Token) -> Element<'a, Message> {
        let label = match token {
            Token::Number(0xA) => "A".to_string(),
            Token::Number(0xB) => "B".to_string(),
            Token::Number(0xC) => "C".to_string(),
            Token::Number(0xD) => "D".to_string(),
            Token::Number(0xE) => "E".to_string(),
            Token::Number(0xF) => "F".to_string(),
            _ => token.to_string(),
        };
        
        button(Text::new(label))
            .on_press(Message::ButtonPressed(token))
            .width(Length::Fill)
            .padding(16)
            .into()
    }

    pub fn new() -> Self {
        HexCalcState {
            calculator: Calculator::new().with_base(16),
            display_content: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed(token) => {
                if token.is_valid_for_base(16) {
                    self.calculator.add_token(token.clone());
                    if let Token::Equals = token {
                        let result = self.calculator.evaluate();
                        self.display_content = format!("{:X}", result);
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
                    HexCalcState::button(Token::ClearScreen),
                    HexCalcState::button(Token::LeftParenthesis),
                    HexCalcState::button(Token::RightParenthesis),
                    HexCalcState::button(Token::Operator(Operator::Division)),
                    HexCalcState::button(Token::ClearToken),
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    HexCalcState::button(Token::Number(7)),
                    HexCalcState::button(Token::Number(8)),
                    HexCalcState::button(Token::Number(9)),
                    HexCalcState::button(Token::Operator(Operator::Multiplication))
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    HexCalcState::button(Token::Number(4)),
                    HexCalcState::button(Token::Number(5)),
                    HexCalcState::button(Token::Number(6)),
                    HexCalcState::button(Token::Operator(Operator::Subtraction))
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    HexCalcState::button(Token::Number(1)),
                    HexCalcState::button(Token::Number(2)),
                    HexCalcState::button(Token::Number(3)),
                    HexCalcState::button(Token::Operator(Operator::Addition))
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    HexCalcState::button(Token::Number(0xA)),
                    HexCalcState::button(Token::Number(0xB)),
                    HexCalcState::button(Token::Number(0xC)),
                    HexCalcState::button(Token::Operator(Operator::Multiplication))
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    HexCalcState::button(Token::Number(0xD)),
                    HexCalcState::button(Token::Number(0xE)),
                    HexCalcState::button(Token::Number(0xF)),
                    HexCalcState::button(Token::Operator(Operator::Subtraction))
                ]
                .spacing(space * 2)
                .padding(space),
                row![
                    HexCalcState::button(Token::Number(0)),
                    HexCalcState::button(Token::Equals)
                ]
                .spacing(space * 2)
                .padding(space),
            ]
            .padding(10)
        ]
        .into()
    }
}