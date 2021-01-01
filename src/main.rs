extern crate json;
extern crate tungstenite;
extern crate url;

use iced::{
    button, executor, scrollable, text_input, window, Align::Center, Application, Column, Command,
    Container, Element, HorizontalAlignment, Length, Scrollable, Settings, Text, TextInput,
};
use tungstenite::*;

pub fn main() -> iced::Result {
    std::thread::spawn(move || {
        connect();
    });
    let mut settings: Settings<()> = Settings::default();
    settings.window = window::Settings {
        transparent: true,
        ..Default::default()
    };
    MainView::run(Settings::default())
}

#[derive(Clone, Debug, Default)]
struct MainView {
    btn_states: BtnStates,
    text_states: TextAreaStates,
    text_values: TextAreaValues,
    scroll_states: ScrollableStates,
    messages: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default)]
struct BtnStates {
    append_btn_state: button::State,
    max_btn_state: button::State,
    min_btn_state: button::State,
    close_btn_state: button::State,
}

#[derive(Clone, Debug, Default)]
struct TextAreaStates {
    message_send_state: text_input::State,
}

#[derive(Clone, Debug, Default)]
struct TextAreaValues {
    message_send_value: String,
}

#[derive(Clone, Debug, Default)]
struct ScrollableStates {
    main_scrollable_state: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Msgs {
    CloseWindow,
    LoadingMessage,
    LoadedMessage(String),
    SendMessage(String),
    SendStage(String),
}

impl Application for MainView {
    type Executor = executor::Default;
    type Message = Msgs;
    type Flags = ();

    fn new(_flags: ()) -> (MainView, Command<Self::Message>) {
        (
            MainView {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("with myself | Oh no!!!!!!!!!!!")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Msgs::CloseWindow => {
                std::process::exit(0);
            }
            Msgs::LoadedMessage(h) => {
                self.messages.push(h);
                println!("{:?}", self.messages);
                Command::none()
            }
            Msgs::LoadingMessage => Command::none(),
            Msgs::SendStage(val) => {
                if val.is_empty() || val.len() == 0 {
                    return Command::none();
                } else {
                    self.text_values.message_send_value = val.clone();
                    Command::none()
                }
            }
            Msgs::SendMessage(val) => {
                if val.is_empty() || val.len() == 0 {
                    println!("Bad value");
                    return Command::none();
                }
                println!("Good value");
                self.update(Msgs::LoadedMessage(val.trim().to_string().clone()));
                self.text_values.message_send_value.clear();
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let main_body: Element<_> = Column::new()
            .padding(20)
            .align_items(Center)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .push(
                Text::new("VELD'S CHAT")
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(100),
            )
            .into();
        let messages: Element<_> = self
            .messages
            .iter_mut()
            .enumerate()
            .fold(
                Column::new().width(Length::Fill).padding(20).spacing(20),
                |column, (_, new_message)| {
                    column.push(
                        Text::new(format!("{}", new_message))
                            .size(35)
                            .horizontal_alignment(HorizontalAlignment::Left),
                    )
                },
            )
            .into();

        let inputs: Element<_> = TextInput::new(
            &mut self.text_states.message_send_state,
            "Send a message",
            &self.text_values.message_send_value,
            Msgs::SendStage,
        )
        .on_submit(Msgs::SendMessage(
            self.text_values.message_send_value.clone(),
        ))
        .size(30)
        .width(Length::Fill)
        .padding(35)
        .into();

        let msg_scroll: Element<_> = Scrollable::new(&mut self.scroll_states.main_scrollable_state)
            .push(messages)
            .width(Length::Fill)
            .height(Length::FillPortion(4))
            .max_height(100)
            .into();

        let content = Column::new()
            .align_items(Center)
            .spacing(20)
            .push(main_body)
            .push(msg_scroll)
            .push(inputs);

        Container::new(content).into()
    }
}

fn connect() {
    let (mut socket, res) = client::connect(url::Url::parse("wss://api.veld.chat").unwrap())
        .expect("Can't connect to server");
    let login_payload = Message::Binary("{\"t\": 0, \"d\": {\"token\":null}}".into());
    println!("{}", login_payload);
    socket.write_message(login_payload).unwrap();
    println!("Connected with http status {}", res.status());
    loop {
        let msg = socket.read_message().expect("Could not read message");
        println!("{}", msg);
    }
}
