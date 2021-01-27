extern crate json;
extern crate reqwest;
extern crate tokio;
extern crate tungstenite;
extern crate url;

use iced::{
    button, executor, scrollable, text_input, window, Align::Center, Application, Column, Command,
    Container, Element, HorizontalAlignment, Length, Scrollable, Settings, Text, TextInput,
};

use tungstenite::*;
use reqwest::header::*;
use std::sync::*;

#[tokio::main]
pub async fn main() -> iced::Result {
    let mut con = Connection::new();
    con.connect().await;
    con.send_message("This is a test".to_string()).await.unwrap();
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
    LoadedMessage(String),
    SendMessage(String),
    SendStage(String),
}

#[derive(Debug, Clone)]
pub struct Connection where Connection: 'static {
    token: String,
}

pub enum Handlers {
    SetToken(String),
    Send(String),
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
                Command::none()
            }
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
                    return Command::none();
                }
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

pub fn heartbeat(
    socket: &'static mut tungstenite::WebSocket<tungstenite::stream::Stream<std::net::TcpStream, native_tls::TlsStream<std::net::TcpStream>>>,
) {
    std::thread::spawn(move || {
        socket.write_message(Message::Binary("{\"t\": 1000, \"d\":null}".into())).unwrap();
        println!("{}", socket.read_message().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(10));
    });
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            token: String::new(),
        }
    }

    pub async fn handle(&mut self, msg: Handlers) {
        match msg {
            Handlers::Send(content) => {
                self.send_message(content).await.unwrap();
            }
            Handlers::SetToken(token) => {
                self.token = token;
            }
        }
    }

    pub async fn connect(
        &mut self,
    ) {
        let (mut socket, _res) = client::connect(url::Url::parse("wss://api.veld.chat").unwrap())
            .expect("Unable to build client");

        let login_payload = Message::Binary("{\"t\": 0, \"d\": {\"token\":null}}".into());
        socket.write_message(login_payload).unwrap();
        let obj = socket.read_message().unwrap();
        let token = json::parse(obj.to_text().unwrap())
            .unwrap()
            .remove("d")
            .remove("token");
        self.token = format!("{}", token);

        let socket = Arc::new(Mutex::new(socket));

        let cl_socket = Arc::clone(&socket);

        std::thread::spawn(move || {
            let mut socket = cl_socket.lock().unwrap();
            println!("Thread A spawned");
            loop {
                println!("{}", socket.read_message().expect("Unable to send socket message"));
            }
        });

        let cl_socket = Arc::clone(&socket);

        std::thread::spawn(move || {
            let mut socket = cl_socket.lock().unwrap();
            println!("thread B spawned");
            loop {
                socket.write_message(Message::Binary("{\"t\": 1000, \"d\":null}".into())).unwrap();
                println!("{}", socket.read_message().unwrap());
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    }

    pub async fn send_message(&mut self, content: String) -> std::result::Result<(), reqwest::Error> {
        let token = format!("Bearer {}", self.token);
        let mut msg_payload = std::collections::HashMap::new();
        msg_payload.insert("content", content);
        println!("{:?}", msg_payload);
        let client = reqwest::blocking::Client::new();
        let post = client
            .post(url::Url::parse("https://api.veld.chat/channels/1/messages").unwrap())
            .header(AUTHORIZATION, token)
            .header(CONTENT_TYPE, "application/json")
            .json(&msg_payload)
            .send()?;
    
        println!("{:#?}", post);

        Ok(())
    }
}
