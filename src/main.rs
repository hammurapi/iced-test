use iced::{executor, widget::text_input, Application, Command, Settings, Theme};

struct MyApplication {
    text: String,
}

#[derive(Debug, Clone)]
enum MyMessage {
    Edit(String),
    Sumbitted,
}

impl Application for MyApplication {
    type Message = MyMessage;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_falgs: Self::Flags) -> (MyApplication, Command<MyMessage>) {
        (
            Self {
                text: String::from("Test"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("iced test")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<MyMessage> {
        match message {
            MyMessage::Edit(updated_text) => self.text = updated_text,
            MyMessage::Sumbitted => {}
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // text("Hello world").into()
        text_input("", &self.text)
            .on_input(MyMessage::Edit)
            .on_submit(MyMessage::Sumbitted)
            .into()
    }
}

fn main() -> iced::Result {
    // println!("Hello, world!");

    MyApplication::run(Settings::default())
}
