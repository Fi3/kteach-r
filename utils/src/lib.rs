use kteach_core::decoder::decode_source;
use kteach_core::engine::Engine;
use kteach_core::midi::midir::{MidiOutput, MidiOutputConnection};
use kteach_core::midi::wmidi::{Channel, MidiMessage as MidiMessage_, U7};
use kteach_core::modules::player::{Player as CorePlayer, PlayerState as CorePlayerState};
use nfd::Response;
use std::fs::File;
use synthesizer_io_core::graph::SetParam;

pub fn load_core_player(
    mut engine: &mut Engine,
    path: &String,
    audio_in_out: &[(usize, usize)],
    is_root: bool,
) -> u8 {
    let file = File::open(path.clone()).unwrap();
    let track = decode_source(file, Some("mp3"));
    let player = CorePlayer::new(track.clone(), None, Some(CorePlayerState::Pause));
    let module = Box::new(player);
    let id: u8;
    if is_root {
        id = engine.add_root(module, audio_in_out) as u8;
    } else {
        id = engine.add_module(module, audio_in_out) as u8;
    }
    register_midi_map(&mut engine, id);
    id
}

pub fn get_play_pause_midi_messages<'a>(id: u8) -> (MidiMessage_<'a>, MidiMessage_<'a>) {
    let play: MidiMessage_;
    let pause: MidiMessage_;

    unsafe {
        play = MidiMessage_::ControlChange(
            Channel::from_index(0).unwrap(),
            U7::from_unchecked(id).into(),
            U7::from_unchecked(0),
        );
        pause = MidiMessage_::ControlChange(
            Channel::from_index(0).unwrap(),
            U7::from_unchecked(id).into(),
            U7::from_unchecked(1),
        );
    }
    (play, pause)
}

pub fn register_midi_map(engine: &mut Engine, id: u8) {
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

pub fn play(midi_out: &mut MidiOutputConnection, id: u8) {
    let (play, _) = get_play_pause_midi_messages(id);
    let mut play_ = [0_u8; 20];
    play.copy_to_slice(&mut play_).unwrap();
    let play_ = &play_[..play.bytes_size()];
    let _ = midi_out.send(play_);
}

pub fn pause(midi_out: &mut MidiOutputConnection, id: u8) {
    let (_, pause) = get_play_pause_midi_messages(id);
    let mut pause_ = [0_u8; 20];
    pause.copy_to_slice(&mut pause_).unwrap();
    let pause_ = &pause_[..pause.bytes_size()];
    let _ = midi_out.send(pause_);
}

pub fn get_midi_out() -> MidiOutputConnection {
    let midi_out = MidiOutput::new("").unwrap();

    let out_ports = midi_out.ports();
    let out_port = &out_ports[0];

    let conn_out = midi_out.connect(&out_port, "").unwrap();
    conn_out
}
pub fn get_path() -> Option<String> {
    let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
        panic!(e);
    });
    match result {
        Response::Okay(path) => Some(path),
        Response::OkayMultiple(_) => None,
        Response::Cancel => None,
    }
}
