use kteach_core::decoder::decode_source;
use kteach_core::modules::player::{Player, PlayerState};
use kteach_core::output::run_cpal;
use std::fs::File;
use std::thread;
use std::time::Duration;
use synthesizer_io_core::graph::{Message, Node, SetParam};
use synthesizer_io_core::worker::Worker;

fn main() {
    let file = File::open("/home/user/Music/audio-prova.mp3").unwrap();
    let track = decode_source(file, Some("mp3"));
    let player = Player::new(track.clone(), None, Some(PlayerState::Play));

    let (worker, tx, rx) = Worker::create(1024);

    let module = Box::new(player);
    tx.send(Message::Node(Node::create(module, 0, [], [])));
    thread::spawn(move || run_cpal(worker));
    let mut controller = 1;
    loop {
        let param_ix = controller % 2;
        controller = controller + 1;
        thread::sleep(Duration::from_secs(1));
        let recv = rx.recv();
        for item in recv {
            match item {
                Message::SetParam(x) => println!("Message recived id: {}", x.ix),
                _ => panic!(),
            }
        }
        tx.send(Message::SetParam(SetParam {
            ix: 0,
            param_ix,
            val: 0.0,
            timestamp: 0,
        }));
    }
}
