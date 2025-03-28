// main.rs
use iced::{Element, Theme};
mod calculator;
mod types;

//use calculator::Calculator;
use types::{CalculatorMode, Token};

mod binary;
mod hex;
mod regular;

#[derive(Default)]
struct CryoCalc {
    screen: Screen,
    display_content: String,
    mode: CalculatorMode,
}

///This shows the different screens, every screen holds it's own state
///default cannot be derived because that only works for enum unit(empty) variants
enum Screen {
    RegularcalculatorScreen(regular::RegularCalculatorState),
}

impl Default for Screen {
    fn default() -> Self {
        //we make the application start with showing the "regular" calculator screen
        return Screen::RegularcalculatorScreen(regular::RegularCalculatorState::new());
    }
}

#[derive(Debug, Clone)]
enum Message {
    //refers to all messages originating from the regularcalculator screen
    RegularCalculator(regular::Message),
}

pub fn main() -> iced::Result {
    iced::application("CryoCalc", CryoCalc::update, CryoCalc::view)
        .theme(|_| Theme::Oxocarbon)
        .centered()
        .run()
}

impl CryoCalc {
    fn update(&mut self, message: Message) {
        match message {
            //if this message comes from the "regularcalculator" screen, let that screen handle it's own messages
            Message::RegularCalculator(message) => {
                if let Screen::RegularcalculatorScreen(state) = &mut self.screen {
                    //pass the message straight through to the regularcalculator instance ()
                    state.update(message);
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            //if the screen is of a certain type, let it's corresponding instance render that screens view
            Screen::RegularcalculatorScreen(state) => state.view().map(Message::RegularCalculator),
        }
    }
}
