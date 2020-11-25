use kteach_core::decoder::decode_source;
use kteach_core::midi::midir::MidiOutput;
use kteach_core::midi::ringbuf::Producer;
use kteach_core::midi::wmidi::{Channel, MidiMessage as MidiMessage_, U7};
use kteach_core::midi::{MidiEngine, MidiMessage};
use kteach_core::modules::player::{Player, PlayerState};
use kteach_core::output::run_cpal;
use std::fs::File;
use std::thread;
use std::time::Duration;
use synthesizer_io_core::graph::{Message, Node, SetParam};
use synthesizer_io_core::queue::Item;
use synthesizer_io_core::worker::Worker;

fn main() {
    let file = File::open("/home/user/Music/audio-prova.mp3").unwrap();
    let track = decode_source(file, Some("mp3"));
    let player = Player::new(track.clone(), None, Some(PlayerState::Play));

    let (worker, sender, receiver) = Worker::create(1024);

    let (midi_engine, mut message_buffer, control_buffer) = MidiEngine::new(sender.clone());
    midi_engine.connect_all();

    let module = Box::new(player);
    sender.send(Message::Node(Node::create(module, 0, [], [])));

    register_midi_map(control_buffer);

    thread::spawn(move || run_cpal(worker));
    thread::spawn(move || run_midi_output());
    loop {
        let recv = receiver.recv();
        for item in recv {
            match &item {
                Message::SetParam(_) => {
                    let _ = message_buffer.push(Item::make_item(item));
                }
                _ => panic!(),
            }
        }
    }
}

fn get_play_pause_midi_messages<'a>() -> (MidiMessage_<'a>, MidiMessage_<'a>) {
    let play: MidiMessage_;
    let pause: MidiMessage_;

    unsafe {
        play = MidiMessage_::ControlChange(
            Channel::from_index(0).unwrap(),
            U7::from_unchecked(0).into(),
            U7::from_unchecked(0),
        );
        pause = MidiMessage_::ControlChange(
            Channel::from_index(0).unwrap(),
            U7::from_unchecked(1).into(),
            U7::from_unchecked(0),
        );
    }
    (play, pause)
}

fn register_midi_map(mut buffer: Producer<(SetParam, MidiMessage)>) {
    let (play, pause) = get_play_pause_midi_messages();
    let worker_message_play_node_0 = SetParam {
        ix: 0,
        param_ix: 0,
        val: 0.0,
        timestamp: 0,
    };
    let worker_message_pause_node_0 = SetParam {
        ix: 0,
        param_ix: 1,
        val: 0.0,
        timestamp: 0,
    };
    let _ = buffer.push((worker_message_play_node_0, play.into()));
    let _ = buffer.push((worker_message_pause_node_0, pause.into()));
}

fn run_midi_output() {
    let midi_out = MidiOutput::new("My Test Output").unwrap();

    let out_ports = midi_out.ports();
    let out_port = &out_ports[0];

    let mut conn_out = midi_out.connect(&out_port, "midir-test").unwrap();

    let (play, pause) = get_play_pause_midi_messages();

    let mut play_ = [0_u8; 20];
    play.copy_to_slice(&mut play_).unwrap();
    let play_ = &play_[..play.bytes_size()];

    let mut pause_ = [0_u8; 20];
    pause.copy_to_slice(&mut pause_).unwrap();
    let pause_ = &pause_[..pause.bytes_size()];

    let mut controller = 1;
    loop {
        controller = controller + 1;
        thread::sleep(Duration::from_millis(10));
        if (controller % 2) == 0 {
            let _ = conn_out.send(play_);
        } else {
            let _ = conn_out.send(pause_);
        }
    }
}
