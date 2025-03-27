// main.rs
use iced::widget::{button, column, row, text, Text};
use iced::{Element, Length};
mod calculator;
mod types;
use calculator::Calculator;
use types::{CalculatorMode, Token};

#[derive(Default)]
struct CryoCalc {
    calculator: Calculator,
    display_content: String,
    mode: CalculatorMode,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(Token),
}

pub fn main() -> iced::Result {
    iced::run("CryoCalc", CryoCalc::update, CryoCalc::view)
}

impl CryoCalc {
    fn update(&mut self, message: Message) {
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
        }
    }

    fn button<'a>(token: Token) -> Element<'a, Message> {
        button(Text::new(token.to_string()))
            .on_press(Message::ButtonPressed(token))
            .width(Length::Fill)
            .padding(16)
            .into()
    }

    fn view(&self) -> Element<Message> {
        let spacing = 5;
        match self.mode {
            CalculatorMode::Regular => column![
                text(&self.display_content),
                column![
                    row![
                        Self::button(Token::ClearScreen),
                        Self::button(Token::Operator(types::Operator::Division)),
                        Self::button(Token::ClearToken)
                    ]
                    .width(Length::Fill)
                    .spacing(spacing)
                    .padding(spacing),
                    row![
                        Self::button(Token::Number(7)),
                        Self::button(Token::Number(8)),
                        Self::button(Token::Number(9)),
                        Self::button(Token::Operator(types::Operator::Multiplication))
                    ]
                    .width(Length::Fill)
                    .spacing(spacing)
                    .padding(spacing),
                    row![
                        Self::button(Token::Number(4)),
                        Self::button(Token::Number(5)),
                        Self::button(Token::Number(6)),
                        Self::button(Token::Operator(types::Operator::Subtraction))
                    ]
                    .width(Length::Fill)
                    .spacing(spacing)
                    .padding(spacing),
                    row![
                        Self::button(Token::Number(1)),
                        Self::button(Token::Number(2)),
                        Self::button(Token::Number(3)),
                        Self::button(Token::Operator(types::Operator::Addition))
                    ]
                    .width(Length::Fill)
                    .spacing(spacing)
                    .padding(spacing),
                    row![
                        Self::button(Token::Number(0)),
                        Self::button(Token::Number(0)),
                        Self::button(Token::Number(9)),
                        Self::button(Token::Equals)
                    ]
                    .width(Length::Fill)
                    .spacing(spacing)
                    .padding(spacing),
                ]
                .padding(10)
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            _ => text("Mode not implemented yet").into(),
        }
    }
}