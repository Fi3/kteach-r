use kteach_core::decoder::decode_source;
use kteach_core::engine::Engine;
use kteach_core::modules::mixer::Mixer;
use kteach_core::modules::player::{Player, PlayerState};
use std::fs::File;
use std::time::Duration;

fn main() {
    let file = File::open("/home/user/Music/audio-prova.mp3").unwrap();
    let file2 = File::open("/home/user/Music/audio-prova2.mp3").unwrap();

    let track = decode_source(file, Some("mp3"));
    let track2 = decode_source(file2, Some("mp3"));

    let player = Player::new(track.clone(), None, Some(PlayerState::Play));
    let module = Box::new(player);

    println!("created module1");

    let player2 = Player::new(track2.clone(), None, Some(PlayerState::Play));
    let module2 = Box::new(player2);

    println!("created module2");

    let mixer = Mixer {};
    let module3 = Box::new(mixer);

    let mut engine = Engine::new();

    engine.add_root(module3, &[(1, 0), (2, 0)]);
    engine.add_module(module2, &[]);
    engine.add_module(module, &[]);

    loop {
        std::thread::sleep(Duration::from_secs(1))
    }
}
