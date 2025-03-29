use crate::{calculator::Calculator, types::Token, types::Operator};

use iced::{
    widget::{button, column, row, text_input, Button, Text},
    Background, Border, Color, Element, Length, Renderer, Shadow, Theme,
};

use button::{Catalog, Status, Style};

#[derive(Default)]
pub struct DecCalcState {
    calculator: Calculator,
    display_content: String, //practice state, lets see if we can toggle this between true and false from main.rs
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Token),
    //SwitchMode,
    DisplayContentChanged(String),
}

impl DecCalcState {
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
        DecCalcState {
            calculator,
            display_content,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed(token) => {
                self.calculator.add_token(token.clone());
                if let Token::Equals = token {
                    let result = self.calculator.evaluate();
                    self.display_content = result.to_string();
                } else {
                    self.display_content = self.calculator.get_display();
                }
                println!("\n{:?}", self.calculator.token_stream);
            }
            Message::DisplayContentChanged(_) => {
                todo!();
            }
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
                    DecCalcState::button(Token::ClearScreen),
                    DecCalcState::button(Token::LeftParenthesis),
                    DecCalcState::button(Token::RightParenthesis),
                    DecCalcState::button(Token::Operator(Operator::Division)),
                    DecCalcState::button(Token::ClearToken),
                ]
                .width(iced::Length::Fill)
                .spacing(space * 2)
                .padding(space),
                row![
                    DecCalcState::button(Token::Number(7)),
                    DecCalcState::button(Token::Number(8)),
                    DecCalcState::button(Token::Number(9)),
                    DecCalcState::button(Token::Operator(Operator::Multiplication))
                ]
                .width(Length::Fill)
                .spacing(space * 2)
                .padding(space),
                row![
                    DecCalcState::button(Token::Number(4)),
                    DecCalcState::button(Token::Number(5)),
                    DecCalcState::button(Token::Number(6)),
                    DecCalcState::button(Token::Operator(Operator::Subtraction))
                ]
                .width(iced::Length::Fill)
                .spacing(space * 2)
                .padding(space),
                row![
                    DecCalcState::button(Token::Number(1)),
                    DecCalcState::button(Token::Number(2)),
                    DecCalcState::button(Token::Number(3)),
                    DecCalcState::button(Token::Operator(Operator::Addition))
                ]
                .width(iced::Length::Fill)
                .spacing(space * 2)
                .padding(space),
                row![
                    DecCalcState::button(Token::Number(0)),
                    DecCalcState::button(Token::Number(0)),
                    DecCalcState::button(Token::Number(9)),
                    DecCalcState::button(Token::Equals)
                ]
                .width(iced::Length::Fill)
                .spacing(space * 2)
                .padding(space),
            ]
            .padding(10)
        ]
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}
