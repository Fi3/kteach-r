use crate::midi::{MidiEngine, MidiMessage};
use crate::modules::mixer::Mixer;
use crate::modules::root::Root;
use crate::output::run_cpal;
use ringbuf::Producer;
use synthesizer_io_core::graph::{Message as Message_, Node, SetParam};
use synthesizer_io_core::module::Module;
use synthesizer_io_core::queue::{Item, Sender};
use synthesizer_io_core::worker::Worker;

pub struct Ids {
    last_id: i64,
    removed_id: Option<usize>,
}

impl Ids {
    pub fn new() -> Self {
        Ids {
            last_id: 0,
            removed_id: None,
        }
    }

    pub fn new_id(&mut self) -> usize {
        match self.removed_id {
            None => {
                self.last_id = self.last_id + 1;
                self.last_id as usize
            }
            Some(x) => {
                self.removed_id = None;
                x as usize
            }
        }
    }
}

pub struct Engine {
    worker_sender: Sender<Message_>,
    midi_control_buffer: Producer<(SetParam, MidiMessage)>,
    ids: Ids,
    root: Vec<(usize, usize)>,
}

impl Engine {
    pub fn new() -> Self {
        let (worker, worker_sender, worker_receiver) = Worker::create(1024);

        let (midi_engine, mut midi_message_buffer, midi_control_buffer) =
            MidiEngine::new(worker_sender.clone());

        midi_engine.connect_all();

        // Only use to instaciate the graph (it would crash without that) then will be replaced
        // by the first added module
        let module = Box::new(Root {});
        worker_sender.send(Message_::Node(Node::create(module, 0, [], [])));

        std::thread::spawn(move || run_cpal(worker));

        std::thread::spawn(move || loop {
            let recv = worker_receiver.recv();
            for item in recv {
                match &item {
                    Message_::SetParam(_) => {
                        let _ = midi_message_buffer.push(Item::make_item(item));
                    }
                    Message_::Note(_) => {
                        println!("TODO");
                    }
                    _ => println!("TODO"),
                }
            }
        });

        Engine {
            worker_sender,
            midi_control_buffer,
            ids: Ids::new(),
            root: Vec::new(),
        }
    }

    pub fn add_module(
        &mut self,
        module: Box<dyn Module>,
        audio_in_out: &[(usize, usize)],
    ) -> usize {
        let id = self.ids.new_id();
        self.worker_sender
            .send(Message_::Node(Node::create(module, id, audio_in_out, [])));
        id
    }

    pub fn add_root(&mut self, module: Box<dyn Module>, audio_in_out: &[(usize, usize)]) -> usize {
        let mut new_root = Vec::new();
        for link in audio_in_out {
            new_root.push(*link);
        }
        self.root = new_root;
        self.worker_sender
            .send(Message_::Node(Node::create(module, 0, audio_in_out, [])));
        0
    }

    pub fn add(&mut self, module: Box<dyn Module>, audio_in_out: &[(usize, usize)]) -> usize {
        let id = self.ids.new_id();
        self.worker_sender
            .send(Message_::Node(Node::create(module, id, audio_in_out, [])));
        self.root.push((id, 0));
        self.update_root();
        id
    }

    fn update_root(&mut self) {
        let mixer = Box::new(Mixer {});
        let audio_in_out = &self.root[0..];
        self.worker_sender
            .send(Message_::Node(Node::create(mixer, 0, audio_in_out, [])));
    }

    pub fn register_midi_message<T: Into<MidiMessage>>(
        &mut self,
        template: SetParam,
        midi_message: T,
    ) {
        let _ = self
            .midi_control_buffer
            .push((template, midi_message.into()));
    }
}
