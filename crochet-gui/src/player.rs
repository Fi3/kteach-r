use crochet::{Button, Column, Cx, Label, TextBox};
use kteach_core::decoder::decode_source;
use kteach_core::engine::Engine;
use kteach_core::midi::midir::MidiOutputConnection;
use kteach_core::midi::wmidi::{Channel, MidiMessage as MidiMessage_, U7};
use kteach_core::modules::player::{Player as CorePlayer, PlayerState as CorePlayerState};
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::path::Path;
use synthesizer_io_core::graph::SetParam;

enum PlayerState {
    Play,
    Pause,
}

pub struct Player {
    path: String,
    play: PlayerState,
    id: u8,
}

impl Player {
    pub fn new(path: String, id: u8) -> Self {
        Player {
            path,
            play: PlayerState::Pause,
            id,
        }
    }

    pub fn run(&mut self, cx: &mut Cx, midi_out: &mut MidiOutputConnection) {
        Column::new().build(cx, |cx| {
            Label::new(&self.path).build(cx);

            if Button::new("PLAY").build(cx) {
                let (play, _) = get_play_pause_midi_messages(self.id);
                let mut play_ = [0_u8; 20];
                play.copy_to_slice(&mut play_).unwrap();
                let play_ = &play_[..play.bytes_size()];
                let _ = midi_out.send(play_);
                self.play = PlayerState::Play;
            }
            if Button::new("PAUSE").build(cx) {
                let (_, pause) = get_play_pause_midi_messages(self.id);
                let mut pause_ = [0_u8; 20];
                pause.copy_to_slice(&mut pause_).unwrap();
                let pause_ = &pause_[..pause.bytes_size()];
                let _ = midi_out.send(pause_);
                self.play = PlayerState::Pause;
            }
        })
    }
}

impl TryFrom<(&Path, &mut Engine)> for Player {
    type Error = ();

    fn try_from(value: (&Path, &mut Engine)) -> Result<Self, Self::Error> {
        let (path, mut engine) = value;
        let file = File::open(path.clone()).map_err(|_| ())?;
        let track = decode_source(file, Some("mp3"));
        let player = CorePlayer::new(track.clone(), None, Some(CorePlayerState::Play));
        let module = Box::new(player);
        let id = engine.add_module(module) as u8;
        register_midi_map(&mut engine, id);
        Ok(Player::new(path.to_str().unwrap().to_string(), id))
    }
}

#[derive(PartialEq)]
enum AddPlayerState {
    Initial,
    NewTriggered(String),
}

pub struct AddPlayer {
    state: AddPlayerState,
}

impl AddPlayer {
    pub fn new() -> Self {
        AddPlayer {
            state: AddPlayerState::Initial,
        }
    }

    pub fn run(&mut self, cx: &mut Cx, players: &mut Vec<Player>, mut engine: &mut Engine) {
        Column::new().build(cx, |cx| match self.state {
            AddPlayerState::Initial => {
                if Button::new("NEW").build(cx) {
                    self.state = AddPlayerState::NewTriggered("".to_string());
                }
            }
            AddPlayerState::NewTriggered(ref mut path) => {
                if path.contains("mp3") && std::path::Path::new(&path).exists() {
                    if Button::new("LOAD").build(cx) {
                        let player = (Path::new(&path), engine).try_into();
                        players.push(player.unwrap());
                        self.state = AddPlayerState::Initial;
                    } else {
                        if let Some(path) = TextBox::new(path.clone()).build(cx) {
                            self.state = AddPlayerState::NewTriggered(path);
                        }
                    }
                } else {
                    if let Some(path) = TextBox::new(path.clone()).build(cx) {
                        self.state = AddPlayerState::NewTriggered(path);
                    }
                }
            }
        });
    }
}

fn get_play_pause_midi_messages<'a>(id: u8) -> (MidiMessage_<'a>, MidiMessage_<'a>) {
    let play: MidiMessage_;
    let pause: MidiMessage_;

    unsafe {
        play = MidiMessage_::ControlChange(
            Channel::from_index(0).unwrap(),
            U7::from_unchecked(0).into(),
            U7::from_unchecked(id),
        );
        pause = MidiMessage_::ControlChange(
            Channel::from_index(0).unwrap(),
            U7::from_unchecked(1).into(),
            U7::from_unchecked(id),
        );
    }
    (play, pause)
}

fn register_midi_map(engine: &mut Engine, id: u8) {
    let (play, pause) = get_play_pause_midi_messages(id);
    let worker_message_play_node_0 = SetParam {
        ix: id as usize,
        param_ix: 0,
        val: 0.0,
        timestamp: 0,
    };
    let worker_message_pause_node_0 = SetParam {
        ix: id as usize,
        param_ix: 1,
        val: 0.0,
        timestamp: 0,
    };
    engine.register_midi_message(worker_message_play_node_0, play);
    engine.register_midi_message(worker_message_pause_node_0, pause);
}
