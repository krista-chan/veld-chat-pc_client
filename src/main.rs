use iced::{
    button, executor, scrollable, text_input, window, Align::Center, Application, Button, Column,
    Command, Container, Element, HorizontalAlignment, Length, Scrollable, Settings, Text,
    TextInput,
};
use reqwest::blocking::get;

pub fn main() -> iced::Result {
    let mut settings: Settings<()> = Settings::default();
    println!("{:?}", settings);
    settings.window = window::Settings {
        size: (800, 600),
        ..Default::default()
    };
    MainView::run(settings)
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
    MaximiseWindow,
    MinimiseWindow,
    LoadedMessage(String),
    SendMessage(String),
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
            Msgs::MaximiseWindow => Command::none(),
            Msgs::MinimiseWindow => Command::none(),
            Msgs::LoadedMessage(h) => {
                self.messages.push(h);
                println!("{:?}", self.messages);
                Command::none()
            }
            Msgs::LoadingMessage => {
                self.update(Msgs::LoadedMessage(format!(
                    "{:?}",
                    get_the_page().unwrap()
                )));
                Command::none()
            }
            Msgs::SendMessage(val) => {
                self.text_values.message_send_value = val;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let main_body: Element<_> = Column::new()
            .padding(20)
            .align_items(Center)
            .width(Length::Fill)
            .push(
                Text::new("Here is some text lmao")
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(100),
            )
            .push(
                Button::new(
                    &mut self.btn_states.append_btn_state,
                    Text::new("Append le h"),
                )
                .on_press(Msgs::LoadingMessage),
            )
            .into();
        let messages: Element<_> = self
            .messages
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |column, (_, task)| {
                column.push(Text::new(format!("{:?}", task)))
            })
            .into();
        let inputs: Element<_> = TextInput::new(
            &mut self.text_states.message_send_state,
            "Send a message",
            &self.text_values.message_send_value,
            Msgs::SendMessage,
        )
        .into();
        let content = Column::new()
            .align_items(Center)
            .spacing(20)
            .push(main_body)
            .push(messages)
            .push(inputs);
        let container: Element<_> = Container::new(content).into();
        Scrollable::new(&mut self.scroll_states.main_scrollable_state)
            .push(container)
            .into()
    }
}

pub fn get_the_page(
) -> std::result::Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    let res = get("https://httpbin.org/ip")?.json::<std::collections::HashMap<String, String>>()?;
    Ok(res)
}
