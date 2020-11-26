use kteach_core::decoder::decode_source;
use kteach_core::engine::Engine;
use kteach_core::modules::gain::Gain;
use kteach_core::modules::player::{Player, PlayerState};
use kteach_utils::{gain as set_gain, get_midi_out, register_midi_map_gain};
use std::fs::File;
use std::time::Duration;

fn main() {
    let file = File::open("/home/user/Music/audio-prova.mp3").unwrap();
    let track = decode_source(file, Some("mp3"));
    let player = Player::new(track.clone(), None, Some(PlayerState::Play));
    let player = Box::new(player);

    let gain = Gain::new();
    let gain = Box::new(gain);

    let mut engine = Engine::new();

    let id = engine.add_root(gain, &[(1, 0)]);
    engine.add_module(player, &[]);

    register_midi_map_gain(&mut engine, id as u8);

    let mut out = get_midi_out();

    let mut controller = 1;
    loop {
        std::thread::sleep(Duration::from_millis(100));
        set_gain(&mut out, id as u8, controller % 128);
        controller = controller + 1;
    }
}
