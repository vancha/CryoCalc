use crate::{Token, calculator::Calculator };
use crate::types::Operator;

use iced::{ Element, Length,
        widget::{ button, column, row, Text, text_input }
};

pub struct RegularCalculatorState {
    calculator: Calculator,
    display_content: String,//practice state, lets see if we can toggle this between true and false from main.rs
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Token),
    //SwitchMode,
    DisplayContentChanged(String)
}

impl RegularCalculatorState {
    fn button<'a>(token: Token) -> Element<'a, Message> {
        let str_from_token = token.to_string();
        match token {
            _ => button(Text::new(str_from_token))
                .on_press(Message::ButtonPressed(token))
                .width(Length::Fill)
                .padding(16)
                .into(),
        }
    }

    pub fn new() -> Self {
        let calculator = Calculator::new();
        let display_content = "".to_string();
        RegularCalculatorState {  calculator, display_content  }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            //Message::SwitchMode => {
                /* might not even be handled here, because this message is sent straight to parent view and never gets sent back here*/
            //},
            Message::ButtonPressed(token) => {
                self.calculator.add_token(token.clone());
                if let Token::Equals = token {
                    let result = self.calculator.evaluate();
                    self.display_content = result.to_string();
                } else {
                    self.display_content = self.calculator.get_display();
                }
                println!("\n{:?}", self.calculator.token_stream);
            },
            Message::DisplayContentChanged(_) => {
                todo!();
            },
        }
    }
    pub fn view(&self) -> Element<Message> {
        let spacing = 5;
        column![
                    column![
                        // Example, shows a text input widget w
                        text_input("...", &self.display_content)
                            .on_input(Message::DisplayContentChanged),
                    ],
                    column![
                        row![
                            RegularCalculatorState::button(Token::ClearScreen),
                            RegularCalculatorState::button(Token::LeftParenthesis),
                            RegularCalculatorState::button(Token::RightParenthesis),
                            RegularCalculatorState::button(Token::Operator(Operator::Division)),
                            RegularCalculatorState::button(Token::ClearToken)
                        ]
                        .width(iced::Length::Fill)
                        .spacing(spacing)
                        .padding(spacing),
                        row![
                            RegularCalculatorState::button(Token::Number(7)),
                            RegularCalculatorState::button(Token::Number(8)),
                            RegularCalculatorState::button(Token::Number(9)),
                            RegularCalculatorState::button(Token::Operator(Operator::Multiplication))
                        ]
                        .width(iced::Length::Fill)
                        .spacing(spacing)
                        .padding(spacing),
                        row![
                            RegularCalculatorState::button(Token::Number(4)),
                            RegularCalculatorState::button(Token::Number(5)),
                            RegularCalculatorState::button(Token::Number(6)),
                            RegularCalculatorState::button(Token::Operator(Operator::Subtraction))
                        ]
                        .width(iced::Length::Fill)
                        .spacing(spacing)
                        .padding(spacing),
                        row![
                            RegularCalculatorState::button(Token::Number(1)),
                            RegularCalculatorState::button(Token::Number(2)),
                            RegularCalculatorState::button(Token::Number(3)),
                            RegularCalculatorState::button(Token::Operator(Operator::Addition))
                        ]
                        .width(iced::Length::Fill)
                        .spacing(spacing)
                        .padding(spacing),
                        row![
                            RegularCalculatorState::button(Token::Number(0)),
                            RegularCalculatorState::button(Token::Number(0)),
                            RegularCalculatorState::button(Token::Number(9)),
                            RegularCalculatorState::button(Token::Equals)
                        ]
                        .width(iced::Length::Fill)
                        .spacing(spacing)
                        .padding(spacing),
                    ]
                    .padding(10)
                ]
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into()
    }
}
