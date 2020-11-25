mod player;
use iced::{button, Button, Column, Container, Element, Length, Row, Sandbox, Settings, Text};
use kteach_core::engine::Engine;
use kteach_core::midi::midir::MidiOutputConnection;
use kteach_utils::{get_midi_out, get_path};
use player::Player;

pub fn main() -> iced::Result {
    Kteach::run(Settings::default())
}

//#[derive(Debug)]
struct Kteach {
    players: Vec<Player>,
    add: button::State,
    engine: Engine,
    midi_out: MidiOutputConnection,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddPlayer,
    Play(usize),
    Pause(usize),
}

impl Sandbox for Kteach {
    type Message = Message;

    fn new() -> Self {
        let engine = Engine::new();
        let midi_out = get_midi_out();
        Kteach {
            players: Vec::new(),
            add: button::State::new(),
            engine,
            midi_out,
        }
    }

    fn title(&self) -> String {
        format!("Kteach")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddPlayer => {
                let path = get_path();
                match path {
                    Some(path) => {
                        let index = self.players.len();
                        self.players
                            .push(Player::new(path, index, &mut self.engine));
                    }
                    None => (),
                }
            }
            Message::Pause(index) => {
                self.players[index].pause(&mut self.midi_out);
            }
            Message::Play(index) => {
                self.players[index].play(&mut self.midi_out);
            }
        }
    }
    fn view(&mut self) -> Element<Message> {
        let add_button = Button::new(&mut self.add, Text::new("ADD"))
            .on_press(Message::AddPlayer)
            .style(style::Button);
        let mut player_row = Row::new();
        for player in &mut self.players {
            player_row = player_row.push(player.view());
        }
        let content = Column::new().push(add_button).push(player_row);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}

pub mod style {
    use iced::{button, container, Color};

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::BLACK.into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }

    pub struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Color::WHITE.into(),
                border_radius: 3.0,
                text_color: Color::BLACK,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: Color::WHITE.into(),
                text_color: Color::BLACK,
                ..self.active()
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                border_width: 1.0,
                border_color: Color::BLACK,
                ..self.hovered()
            }
        }
    }
}
