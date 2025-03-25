use iced::widget::{button, column, row, text, text_input};
use iced::Element;

/// This is the struct that will eventually be able to parse the expressions
/// Through something like  ```let result = ExpressionParser::Parse("1 + 5 / (9 * 3)");```
struct ExpressionParser {}

#[derive(Default)]
enum CalculatorMode {
    #[default]
    Regular,
    Hexadecimal,
    Binary,
}

/// This struct, holds the state of the entire application
#[derive(Default)]
struct CryoCalc {
    //holds the state for our display
    display_content: String,
    //used to decide what to display
    mode: CalculatorMode,
}

pub fn main() -> iced::Result {
    iced::run("CryoCalc", CryoCalc::update, CryoCalc::view)
}

impl CryoCalc {
    fn update(self: &mut CryoCalc, message: Message) {
        match message {
            //gets triggered when typing expressions in the calculators display
            Message::DisplayContentChanged(content) => {
                self.display_content = content;
            },
            Message::ButtonPressed(value) => {
                self.display_content.push_str(&value);
            },
        }
    }

    fn view(self: &CryoCalc) -> Element<Message> {
       match self.mode {
           _ => {
                column![
                    //example, shows a text input widget w
                    text_input("...", &self.display_content)
                        .on_input(Message::DisplayContentChanged),
                    button("1").on_press(Message::ButtonPressed(String::from("1")))
                ].into()
           },
       }
    }
}

#[derive(Debug, Clone)]
enum Message {
    DisplayContentChanged(String),
    ButtonPressed(String)
    //Likely this needs some kind of button pressed message that tells us which button has been
    //pressed?
}
