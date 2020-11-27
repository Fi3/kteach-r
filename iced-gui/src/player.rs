use crate::style;
use crate::Message;
use iced::{button, slider, Button, Column, Row, Slider, Text};
use kteach_core::engine::Engine;
use kteach_core::midi::midir::MidiOutputConnection;
use kteach_utils::{gain, load_core_player, pause, play};
use std::path::Path;

#[derive(Debug)]
pub struct Player {
    path: String,
    play: button::State,
    pause: button::State,
    index: usize,
    id: u8,
    gain: u8,
    gain_id: u8,
    slider: slider::State,
}

impl Player {
    pub fn new(path: String, index: usize, engine: &mut Engine) -> Self {
        let (id, gain_id) = load_core_player(engine, &path);
        Player {
            path,
            play: button::State::new(),
            pause: button::State::new(),
            id,
            index,
            gain_id,
            gain: 127,
            slider: slider::State::new(),
        }
    }

    pub fn view(&mut self) -> Column<Message> {
        let index = self.index;

        let filename = Path::new(&self.path).file_name().unwrap().to_str().unwrap();

        let label = Text::new(filename);

        let play = Button::new(&mut self.play, Text::new("PLAY"))
            .on_press(Message::Play(index))
            .style(style::Button);

        let pause = Button::new(&mut self.pause, Text::new("PAUSE"))
            .on_press(Message::Pause(index))
            .style(style::Button);

        let gain = Slider::new(&mut self.slider, 0..=127, self.gain, move |gain| {
            Message::Gain(gain, index)
        })
        .step(1);

        Column::new()
            .push(label)
            .push(Row::new().push(play).push(pause))
            .push(gain)
            .max_width(150)
    }

    pub fn play(&self, midi_out: &mut MidiOutputConnection) {
        play(midi_out, self.id);
    }

    pub fn pause(&self, midi_out: &mut MidiOutputConnection) {
        pause(midi_out, self.id);
    }

    pub fn gain(&mut self, midi_out: &mut MidiOutputConnection, gain_: u8) {
        self.gain = gain_;
        gain(midi_out, self.gain_id, self.gain);
    }
}
