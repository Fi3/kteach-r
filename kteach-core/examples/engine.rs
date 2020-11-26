use kteach_core::decoder::decode_source;
use kteach_core::engine::Engine;
use kteach_core::modules::player::{Player, PlayerState};
use std::fs::File;
use std::time::Duration;

fn main() {
    let file = File::open("/home/user/Music/audio-prova.mp3").unwrap();
    let track = decode_source(file, Some("mp3"));
    let player = Player::new(track.clone(), None, Some(PlayerState::Play));
    let module = Box::new(player);

    let mut engine = Engine::new();

    let id = engine.add_module(module, &[]);
    println!("{}", id);

    loop {
        std::thread::sleep(Duration::from_secs(1))
    }
}
