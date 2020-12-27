use iced::{
    button, executor, Align::Center, Application, Button, Command, Element, Row, Settings, Text,
};
use reqwest::blocking::{get};

fn main() -> iced::Result {
    Hello::run(Settings::default())
}

#[derive(Default)]
struct Hello {
    button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonPresses {
    Press1,
}

impl Application for Hello {
    type Executor = executor::Default;
    type Message = ButtonPresses;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (
            Hello {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            ButtonPresses::Press1 => Command::perform(get_the_page(), move |res| {
                println!("{:?}", res.unwrap());
                message
            }),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Row::new()
            .padding(20)
            .align_items(Center)
            .push(
                Button::new(
                    &mut self.button_state,
                    Text::new("This is a button biiiiitch"),
                )
                .on_press(self::ButtonPresses::Press1),
            )
            .into()
    }
}

pub async fn get_the_page() -> std::result::Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    let res = get("https://httpbin.org/ip")?
        .json::<std::collections::HashMap<String, String>>()?;
    println!("{:?}", res);
    Ok(res)
}
