use iced::widget::button::{Catalog, Status, Style};
use iced::{Background, Color};


pub struct MyTheme;

#[derive(Debug, Default)]
pub enum ButtonClass {
    #[default]
    Primary,
    Secondary,
    Danger,
}


impl Catalog for MyTheme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let mut style = Style::default();

        match class {
            ButtonClass::Primary => {
                match status {
                    Status::Hovered => {
                         style.background = Some(Background::Color(Color::from_rgb(1.0, 0.0, 0.0)));
                    },
                    _ => {
                         style.background = Some(Background::Color(Color::from_rgb(0.529, 0.808, 0.921)));
                    },
                }

            }
            ButtonClass::Secondary => {
                style.background = Some(Background::Color(Color::WHITE));
            }
            ButtonClass::Danger => {
                style.background = Some(Background::Color(Color::from_rgb(0.941, 0.502, 0.502)));
            }
        }


        style
    }
}
