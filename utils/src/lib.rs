use kteach_core::decoder::decode_source;
use kteach_core::engine::Engine;
use kteach_core::midi::midir::{MidiOutput, MidiOutputConnection};
use kteach_core::midi::wmidi::{Channel, MidiMessage as MidiMessage_, U7};
use kteach_core::modules::gain::Gain;
use kteach_core::modules::player::{Player, PlayerState};
use nfd::Response;
use std::fs::File;
use synthesizer_io_core::graph::SetParam;

pub fn load_core_player(mut engine: &mut Engine, path: &String) -> (u8, u8) {
    // Open and decode audio source
    let file = File::open(path.clone()).unwrap();
    let track = decode_source(file, Some("mp3"));

    // Create and load the player
    let player = Player::new(track.clone(), None, Some(PlayerState::Pause));
    let player = Box::new(player);
    let id = engine.add_module(player, &[]) as u8;
    register_midi_map(&mut engine, id);

    // Create and load the gain of the player
    let gain = Gain::new();
    let gain = Box::new(gain);
    let gain_id = engine.add(gain, &[(id as usize, 0)]) as u8;
    register_midi_map_gain(&mut engine, gain_id);

    (id, gain_id)
}

// TODO channel should be defined in config file
pub fn get_play_pause_midi_messages<'a>(id: u8) -> (MidiMessage_<'a>, MidiMessage_<'a>) {
    let play: MidiMessage_;
    let pause: MidiMessage_;

    unsafe {
        play = MidiMessage_::ControlChange(
            Channel::from_index(15).unwrap(),
            U7::from_unchecked(id).into(),
            U7::from_unchecked(0),
        );
        pause = MidiMessage_::ControlChange(
            Channel::from_index(15).unwrap(),
            U7::from_unchecked(id).into(),
            U7::from_unchecked(1),
        );
    }
    (play, pause)
}

pub fn register_midi_map(engine: &mut Engine, id: u8) {
    let (play, pause) = get_play_pause_midi_messages(id);
    let worker_message_play = SetParam {
        ix: id as usize,
        param_ix: 0,
        val: 0.0,
        timestamp: 0,
    };
    let worker_message_pause = SetParam {
        ix: id as usize,
        param_ix: 1,
        val: 0.0,
        timestamp: 0,
    };
    engine.register_midi_message(worker_message_play, play);
    engine.register_midi_message(worker_message_pause, pause);
}

// TODO channel should be defined in config file
pub fn get_gain_midi_messages<'a>(id: u8, value: u8) -> MidiMessage_<'a> {
    let gain: MidiMessage_;

    unsafe {
        gain = MidiMessage_::ControlChange(
            Channel::from_index(15).unwrap(),
            U7::from_unchecked(id).into(),
            U7::from_unchecked(value).into(),
        );
    }
    gain
}

pub fn register_midi_map_gain(engine: &mut Engine, id: u8) {
    let gain = get_gain_midi_messages(id, 0);
    let worker_message_gain = SetParam {
        ix: id as usize,
        param_ix: 0,
        val: 0.0,
        timestamp: 0,
    };
    engine.register_midi_message(worker_message_gain, gain);
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

pub fn gain(midi_out: &mut MidiOutputConnection, id: u8, value: u8) {
    let gain = get_gain_midi_messages(id, value);
    let mut gain_ = [0_u8; 20];
    gain.copy_to_slice(&mut gain_).unwrap();
    let gain_ = &gain_[..gain.bytes_size()];
    let _ = midi_out.send(gain_);
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
