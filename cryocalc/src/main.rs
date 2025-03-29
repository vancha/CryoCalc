// main.rs
use iced::{
    widget::{button, column, row, text_input, Button, Text},
    Background, Border, Color, Element, Length, Renderer, Shadow, Theme,
};
mod calculator;
mod types;

// use calculator::Calculator;
use types::{CalculatorMode};

mod binary;
mod hex;
mod dec;

//#[derive(Default)]
struct CryoCalc {
    dec_state: dec::DecCalcState,
    bin_state: binary::BinCalcState,
    hex_state: hex::HexCalcState,
    current_mode: CalculatorMode,
}

#[derive(Debug, Clone)]
enum Message {
    DecMessage(dec::Message),
    BinMessage(binary::Message),
    HexMessage(hex::Message),
    CycleMode,
}

impl Default for CryoCalc {
    fn default() -> Self {
        Self {
            dec_state: dec::DecCalcState::new(),
            bin_state: binary::BinCalcState::new(),
            hex_state: hex::HexCalcState::new(),
            current_mode: CalculatorMode::Decimal,
        }
    }
}

// This shows the different screens, every screen holds it's own state
// default cannot be derived because that only works for enum unit(empty) variants
enum Screen {
    DecCalcScreen(dec::DecCalcState),
    
}

impl Default for Screen {
    fn default() -> Self {
        // We make the application start with showing the "regular" calculator screen
        return Screen::DecCalcScreen(dec::DecCalcState::new());
    }
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
            Message::DecMessage(msg) => self.dec_state.update(msg),
            Message::BinMessage(msg) => self.bin_state.update(msg),
            Message::HexMessage(msg) => self.hex_state.update(msg),
            Message::CycleMode => {
                self.current_mode = match self.current_mode {
                    CalculatorMode::Decimal => CalculatorMode::Binary,
                    CalculatorMode::Binary => CalculatorMode::Hex,
                    CalculatorMode::Hex => CalculatorMode::Decimal,
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mode_text = match self.current_mode {
            CalculatorMode::Decimal => "DEC",
            CalculatorMode::Binary => "BIN",
            CalculatorMode::Hex => "HEX",
        };
        
        let mode_button = button(Text::new(format!("Switch Mode ({})", mode_text)))
            .on_press(Message::CycleMode)
            .width(Length::Fill)
            .padding(10);

        let current_view = match self.current_mode {
            CalculatorMode::Decimal => self.dec_state.view().map(Message::DecMessage),
            CalculatorMode::Binary => self.bin_state.view().map(Message::BinMessage),
            CalculatorMode::Hex => self.hex_state.view().map(Message::HexMessage),
        };

        column![
            mode_button,
            current_view
        ]
        .spacing(10)
        .padding(20)
        .into()
    }
}
