use kteach_core::decoder::decode_source;
use kteach_core::modules::player::{Player, PlayerState};
use kteach_core::output::run_cpal;
//use kteach_core::state::State;
use std::fs::File;
use std::thread;
use std::time::Duration;
use synthesizer_io_core::graph::{Message, Node, SetParam};
use synthesizer_io_core::worker::Worker;

///////////////////APPUNTI//////////////////////
/// il primo nodo immesso deve avere ix = 0
/// quelli dopo arbitrario
/// se mando un messaggio set_param per un ix di un nodo non esistante panic!

fn main() {
    // Spwan UI thread
    //
    // Spwan loader thread
    //
    // Spwan MIDI thread
    //
    // Spwan main_loop thread
    //let (initial_state, sender, receiver, free_memory_receiver) = State.initialize();
    //let audio_engine = todo!();
    let file = File::open("/home/user/Music/audio-prova.mp3").unwrap();
    let track = decode_source(file, Some("mp3"));
    let player = Player::new(track.clone(), None, Some(PlayerState::Play));

    let (worker, tx, rx) = Worker::create(1024);

    // Set up working graph; will probably be replaced by the engine before
    // the first audio callback runs.
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
                Message::Node(x) => println!("ITEM: {}", x.ix),
                Message::SetParam(x) => println!("gigix: {}", x.ix),
                Message::Note(_) => println!("NOte"),
                Message::Quit => println!("quit"),
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
