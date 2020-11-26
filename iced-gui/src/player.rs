use crate::style;
use crate::Message;
use iced::{button, Button, Column, Row, Text};
use kteach_core::engine::Engine;
use kteach_core::midi::midir::MidiOutputConnection;
use kteach_utils::{load_core_player, pause, play};
use std::path::Path;

#[derive(Debug)]
pub struct Player {
    path: String,
    play: button::State,
    pause: button::State,
    index: usize,
    id: u8,
}

impl Player {
    pub fn new(path: String, index: usize, engine: &mut Engine) -> Self {
        let id = load_core_player(engine, &path, &[]);
        Player {
            path,
            play: button::State::new(),
            pause: button::State::new(),
            id,
            index,
        }
    }

    pub fn view(&mut self) -> Column<Message> {
        let filename = Path::new(&self.path).file_name().unwrap().to_str().unwrap();
        let label = Text::new(filename);
        let play = Button::new(&mut self.play, Text::new("PLAY"))
            .on_press(Message::Play(self.index))
            .style(style::Button);
        let pause = Button::new(&mut self.pause, Text::new("PAUSE"))
            .on_press(Message::Pause(self.index))
            .style(style::Button);
        Column::new()
            .push(label)
            .push(Row::new().push(play).push(pause))
    }

    pub fn play(&self, midi_out: &mut MidiOutputConnection) {
        play(midi_out, self.id);
    }

    pub fn pause(&self, midi_out: &mut MidiOutputConnection) {
        pause(midi_out, self.id);
    }
}
