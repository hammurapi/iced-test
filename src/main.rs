use iced::{widget::text_input, Sandbox, Settings};

struct MyApplication {
    text: String,
}

#[derive(Debug, Clone)]
enum MyMessage {
    Edit(String),
}

impl Sandbox for MyApplication {
    type Message = MyMessage;

    fn new() -> Self {
        Self {
            text: String::from("Test"),
        }
    }

    fn title(&self) -> String {
        String::from("iced test")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyMessage::Edit(updated_text) => self.text = updated_text,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // text("Hello world").into()
        text_input("", &self.text).on_input(MyMessage::Edit).into()
    }
}

fn main() -> iced::Result {
    // println!("Hello, world!");

    MyApplication::run(Settings::default())
}
