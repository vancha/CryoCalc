use iced::{Element, widget::Text, };

pub struct RegularCalculator {
    state: bool,//practice state, lets see if we can toggle this between true and false from main.rs
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchMode
}

impl RegularCalculator {
    pub fn new() -> Self {
        RegularCalculator { state :  true }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SwitchMode => {
                /* might not even be handled here, because this message is sent straight to parent view and never gets sent back here*/
            },
        }
    }
    pub fn view<'a>(&self) -> Element<'a, Message> {
        Text::new("regular view").into()
    }
}
