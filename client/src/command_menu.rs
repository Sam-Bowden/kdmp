use crate::{error_menu::ErrorMenu, event::Event, option::Option, request::Request, CurrentView};
use iced::{
    color,
    widget::{text, text_input::Id, Button, Column, Container, Scrollable, TextInput},
    Element, Length, Padding,
};

pub struct CommandMenu {
    input: String,
    tracks: Vec<Option>,
    commands: Vec<Option>,
}

pub enum Mode {
    Commands,
    Music,
    Invalid,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    OptionPressed(usize),
    InputConfirmed,
}

impl CommandMenu {
    pub fn new() -> CurrentView {
        let tracks = match Self::get_tracks() {
            Ok(t) => t,
            Err(e) => return CurrentView::ErrorMenu(e),
        };

        CurrentView::CommandMenu(CommandMenu {
            input: String::new(),
            tracks,
            commands: Self::get_commands(),
        })
    }

    pub fn update(&mut self, message: Message) -> Event {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
                Event::None
            }
            Message::OptionPressed(_num) => Event::None,
            Message::InputConfirmed => self.execute(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut components = self.input.split(" ");

        let (mode, param) = match (components.next(), components.next()) {
            (Some("pl"), Some(track)) => (Mode::Music, track),
            (Some(command), None) => (Mode::Commands, command),
            (None, None) => (Mode::Commands, ""),
            _ => (Mode::Invalid, ""),
        };

        let column = Column::new()
            .spacing(10)
            .align_items(iced::Alignment::Center)
            .push(self.view_entry())
            .push(self.view_title(&mode))
            .push(self.view_options(&mode, param));

        Container::new(column)
            .center_x()
            .width(Length::Fill)
            .padding(10)
            .into()
    }

    fn view_entry(&self) -> Element<Message> {
        TextInput::new("Enter command...", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::InputConfirmed)
            .id(Id::new("command_input"))
            .into()
    }

    fn view_title(&self, mode: &Mode) -> Element<Message> {
        match mode {
            Mode::Commands => text("Available commands"),
            Mode::Music => text("Tracks"),
            Mode::Invalid => text("Invalid command").style(color!(0xe55812)),
        }
        .into()
    }

    fn view_options(&self, mode: &Mode, param: &str) -> Element<Message> {
        let mut column = Column::new()
            .align_items(iced::Alignment::Center)
            .padding(Padding::from([0, 20]));

        for (i, entry) in match mode {
            Mode::Commands => self.commands.iter(),
            Mode::Music => self.tracks.iter(),
            Mode::Invalid => [].iter(),
        }
        .filter(|s| s.starts_with(param))
        .enumerate()
        {
            let button = Button::new(entry.display())
                .on_press(Message::OptionPressed(i))
                .width(Length::Fill)
                .style(if i == 0 {
                    iced::theme::Button::Positive
                } else {
                    iced::theme::Button::Primary
                });
            column = column.push(button);
        }

        Scrollable::new(column).into()
    }

    fn get_tracks() -> Result<Vec<Option>, ErrorMenu> {
        let Some(directory) = dirs::audio_dir() else { return Err(ErrorMenu::NoAudioDirectoryConfigured) };

        Ok(
            globwalk::GlobWalkerBuilder::from_patterns(directory, &["*.{mp3,flac,wav}"])
                .build()
                .expect("Failed to build glob walker")
                .into_iter()
                .filter_map(Result::ok)
                .map(|s| Option::new_track(s.path()))
                .collect(),
        )
    }

    fn get_commands() -> Vec<Option> {
        vec![
            Option::new_command("ps (pause)")
                .keyword("ps")
                .keyword("pause"),
            Option::new_command("pl (play)")
                .keyword("pl")
                .keyword("play"),
            Option::new_command("s (stop)").keyword("s").keyword("stop"),
            Option::new_command("r (resume)")
                .keyword("r")
                .keyword("resume"),
            Option::new_command("n (next)").keyword("n").keyword("next"),
            Option::new_command("e (exit)").keyword("e").keyword("exit"),
        ]
    }

    fn execute(&mut self) -> Event {
        let mut command_components = self.input.split(" ");

        let (command, argument) = (command_components.next(), command_components.next());

        let result = match command {
            Some("pl") | Some("play") => {
                if let Some(arg) = argument {
                    if let Some(track) = self.tracks.iter().find(|s| s.starts_with(arg)) {
                        Request::Play(track.get_path().clone()).send();
                    }
                }
                Event::None
            }
            Some("s") | Some("stop") => Request::Stop.send(),
            Some("ps") | Some("pause") => Request::Pause.send(),
            Some("r") | Some("resume") => Request::Resume.send(),
            Some("n") | Some("next") => Request::Next.send(),
            Some("e") | Some("exit") => Event::Exit,
            _ => return Event::None,
        };

        self.input.clear();

        result
    }
}
