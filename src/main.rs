use iced::{widget::text, Sandbox, Settings};

struct MyApplication;

#[derive(Debug)]
enum MyMessage {}

impl Sandbox for MyApplication {
    type Message = MyMessage;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("iced test")
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        text("Hello world").into()
    }
}

fn main() -> iced::Result {
    // println!("Hello, world!");

    MyApplication::run(Settings::default())
}
