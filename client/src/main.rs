use std::time::{Duration, Instant};

use command_menu::CommandMenu;
use error_menu::ErrorMenu;
use event::Event;
use iced::widget::text_input::{self, Id};
use iced::{color, executor, theme};
use iced::{window, Application, Element, Settings, Theme};
use iced_native::command::Action;
use iced_native::{subscription, Command, Subscription};

mod command_menu;
mod communication;
mod error_menu;
mod event;
mod option;

fn main() -> iced::Result {
    KDMP::run(Settings {
        window: window::Settings {
            size: (400, 250),
            resizable: false,
            ..Default::default()
        },
        default_font: Some(include_bytes!("../../resources/hack/Hack-Regular.ttf")),
        ..Default::default()
    })
}

pub enum CurrentView {
    CommandMenu(CommandMenu),
    ErrorMenu(ErrorMenu),
}

struct KDMP {
    current_view: CurrentView,
}

#[derive(Debug, Clone)]
enum Message {
    Command(command_menu::Message),
    Error(error_menu::Message),
    EventOccurred(iced_native::Event),
    Refresh(Instant),
}

impl Application for KDMP {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (KDMP, Command<Self::Message>) {
        (
            KDMP {
                current_view: CommandMenu::new(),
            },
            text_input::focus(Id::new("command_input")),
        )
    }

    fn title(&self) -> String {
        String::from("KDMP")
    }

    fn theme(&self) -> Self::Theme {
        Theme::custom(theme::Palette {
            background: color!(0x002626),
            text: color!(0xefe7da),
            primary: color!(0x0E4749),
            success: color!(0x95c623),
            danger: color!(0xe55812),
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match match (message, &mut self.current_view) {
            (Message::Command(m), CurrentView::CommandMenu(cm)) => cm.update(m),
            (Message::Error(m), CurrentView::ErrorMenu(em)) => em.update(m),
            (Message::EventOccurred(e), CurrentView::ErrorMenu(em)) => {
                em.update(error_menu::Message::EventOccurred(e))
            }
            (Message::EventOccurred(e), CurrentView::CommandMenu(cm)) => {
                cm.update(command_menu::Message::EventOccurred(e))
            }
            (Message::Refresh(_), CurrentView::CommandMenu(cm)) => {
                cm.update(command_menu::Message::Refresh)
            }
            _ => return Command::none(),
        } {
            Event::OpenErrorMenu(e) => {
                self.current_view = CurrentView::ErrorMenu(e);
                Command::none()
            }
            Event::OpenCommandMenu => {
                self.current_view = CommandMenu::new();
                return text_input::focus(Id::new("command_input"));
            }
            Event::Exit => {
                return iced::Command::single(Action::Window(iced_native::window::Action::Close));
            }
            Event::None => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.current_view {
            CurrentView::CommandMenu(cm) => cm.view().map(Message::Command),
            CurrentView::ErrorMenu(em) => em.view().map(Message::Error),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            subscription::events().map(Message::EventOccurred),
            iced::time::every(Duration::from_millis(500)).map(Message::Refresh),
        ])
    }
}
