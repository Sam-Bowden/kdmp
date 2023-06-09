use crate::event::Event;
use iced::{
    color, keyboard,
    widget::{button, text, Column, Container},
    Element, Length,
};
use iced_native::{alignment::Horizontal, keyboard::KeyCode};

#[derive(Debug, Clone)]
pub enum Message {
    OK,
    Exit,
    EventOccurred(iced_native::Event),
}

pub enum ErrorMenu {
    FailedToConnectToDaemon,
    NoAudioDirectoryConfigured,
}

impl ErrorMenu {
    pub fn view(&self) -> Element<Message> {
        let column = Column::new()
            .push(text("Error!").style(color!(0xe55812)))
            .push(self.error_message())
            .push(match self {
                Self::FailedToConnectToDaemon => button("OK (Enter)").on_press(Message::OK),
                Self::NoAudioDirectoryConfigured => button("Exit (Enter)").on_press(Message::Exit),
            })
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center);

        Container::new(column)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Event {
        match message {
            Message::OK => Event::OpenCommandMenu,
            Message::Exit => Event::Exit,
            Message::EventOccurred(event) => match event {
                iced_native::Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: KeyCode::Enter,
                    ..
                }) => match self {
                    Self::FailedToConnectToDaemon => Event::OpenCommandMenu,
                    Self::NoAudioDirectoryConfigured => Event::Exit,
                },
                _ => Event::None,
            },
        }
    }

    fn error_message(&self) -> Element<Message> {
        match self {
            Self::FailedToConnectToDaemon => text("Failed to connect to the daemon. Please refer to the README to ensure that it is running in the background correctly."),
            Self::NoAudioDirectoryConfigured => text("Could not find your music directory - XDG_MUSIC_DIR may not have been set."),
        }.horizontal_alignment(Horizontal::Center).into()
    }
}
