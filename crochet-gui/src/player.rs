use crochet::{Button, Column, Cx, Label};
use kteach_core::engine::Engine;
use kteach_core::midi::midir::MidiOutputConnection;
use kteach_utils::{get_path, load_core_player, pause, play};
use std::convert::{TryFrom, TryInto};

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
            Label::new(
                std::path::Path::new(&self.path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
            .build(cx);

            if Button::new("PLAY").build(cx) {
                play(midi_out, self.id);
                self.play = PlayerState::Play;
            }
            if Button::new("PAUSE").build(cx) {
                pause(midi_out, self.id);
                self.play = PlayerState::Pause;
            }
        })
    }
}

impl TryFrom<(&String, &mut Engine)> for Player {
    type Error = ();

    fn try_from(value: (&String, &mut Engine)) -> Result<Self, Self::Error> {
        let (path, engine) = value;
        let id = load_core_player(engine, path, &[]);
        Ok(Player::new(path.clone(), id))
    }
}

#[derive(PartialEq)]
enum AddPlayerState {
    Initial,
    NewTriggered,
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

    pub fn run(&mut self, cx: &mut Cx, players: &mut Vec<Player>, engine: &mut Engine) {
        Column::new().build(cx, |cx| match self.state {
            AddPlayerState::Initial => {
                if Button::new("NEW").build(cx) {
                    self.state = AddPlayerState::NewTriggered;
                }
            }
            AddPlayerState::NewTriggered => {
                let path = get_path();
                match path {
                    None => (),
                    Some(path) => {
                        let player = (&path, engine).try_into();
                        players.push(player.unwrap());
                        self.state = AddPlayerState::Initial;
                    }
                }
            }
        });
    }
}
