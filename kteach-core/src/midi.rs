use midir::{MidiInput, MidiInputConnection};
//use rb::{Consumer, Producer, RbConsumer, RbProducer, Result, SpscRb, RB};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::boxed::Box;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use synthesizer_io_core::graph::{Graph, Message as Message_, Node};
use synthesizer_io_core::queue::{Item, Queue, Receiver, Sender};
use wmidi::{
    Channel, ControlFunction, ControlValue, MidiMessage, Note, PitchBend, ProgramNumber, Velocity,
};

// arbitraly chosen, TODO use const generics!
const MIDI_MAP_CAPACITY: usize = 10;
const MAX_MIDI_CONNECTIONS: usize = 10;

const NUMBER_OF_MIDI_CHANNEL: usize = 16;
const NUMBER_OF_MIDI_CONTROL_FUNCTIONS: usize = 128; // 2^7 (control functions are 7 bit)
const NUMBER_OF_MIDI_NOTE: usize = 128; // as above

type Message = Item<Message_>;

#[derive(Hash, PartialEq, Eq)]
pub enum MidiMessages {
    NoteOff,
    NoteOn,
    PolyphonicKeyPressure,
    ControlChange,
    ProgramChange,
    ChannelPressure,
    PitchBendChange,
    SysEx,
    OwnedSysEx,
    MidiTimeCode,
    SongPositionPointer,
    SongSelect,
    Reserved,
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
}

struct MidiMap {
    map: Vec<
        HashMap<
            (Option<Note>, Option<ControlFunction>, MidiMessages),
            [Message; MIDI_MAP_CAPACITY],
        >,
    >,
}

impl MidiMap {
    pub fn new() -> Self {
        let mut map = vec![];
        for _ in 0..NUMBER_OF_MIDI_CHANNEL {
            let inner_map =
                HashMap::with_capacity(NUMBER_OF_MIDI_CONTROL_FUNCTIONS + NUMBER_OF_MIDI_NOTE + 1); // questo e' sbagliato!
            map.push(inner_map);
        }
        MidiMap { map }
    }
    pub fn get(
        &mut self,
        channel: &Channel,
        kind: MidiMessages,
        note: &Option<Note>,
        control_function: &Option<ControlFunction>,
    ) -> Option<&mut [Message; 10]> {
        let message = self.map[channel.index() as usize].get_mut(&(*note, *control_function, kind));
        message
    }
}

struct MidiEngine_ {
    midi_connections: Vec<MidiInputConnection<()>>,
    sender: Sender<Message>,
    midi_map: MidiMap,
    map_controller: Receiver<(Message, Channel, Option<Note>, Option<ControlFunction>)>,
    message_buffer: Consumer<Message>,
}

// Take message template from the MidiMap
// And an already allocated Item<Message_> from the buffer
#[inline]
fn process_message<T: Into<f32> + Copy>(
    message_template: &Message,
    mut message: Message,
    val: T,
) -> Message {
    match ((*message_template).deref(), message.deref_mut()) {
        (Message_::SetParam(set_param_template), Message_::SetParam(set_param)) => {
            set_param.ix = set_param_template.ix;
            set_param.param_ix = set_param_template.param_ix;
            set_param.val = val.into();
            set_param.timestamp = 0;
        }
        _ => (),
    }
    message
}

//#[inline]
//fn send_messages(sender: Sender<Message>, messages: &[Message; MIDI_MAP_CAPACITY]) {
//    for message in messages {
//        //let gg = *message;
//        sender.send_item(message);
//    }
//}

impl MidiEngine_ {
    fn handle_message(&mut self, timestamp_micro_seconds: u64, raw_midi_message: &[u8]) {
        let midi_message = MidiMessage::try_from(raw_midi_message).unwrap();
        let mut velocity: Option<Velocity>;
        let mut control_value: Option<ControlValue>;
        let mut program_number: Option<ProgramNumber>;
        let mut pitch_bend: Option<PitchBend>;
        let worker_message = match midi_message {
            MidiMessage::ControlChange(channel, control_function, control_value_) => {
                control_value = Some(control_value_);
                self.midi_map.get(
                    &channel,
                    MidiMessages::ControlChange,
                    &None,
                    &Some(control_function),
                )
            }
            _ => None,
        };
        // TODO worker_message dal ringbuffer di ritorno se non c'e' setta il messaggio to None e
        // vai avanti quindi usare una chiamate che non blocca per recuperarlo
        //match (worker_message, velocity, control_value, program_number, pitch_bend) => {
        //    (None, _, _, _, _) => (),
        //    (Some(worker_message), None, None, None, None) =>
    }
}

pub struct MidiEngine(Arc<Mutex<MidiEngine_>>);

impl MidiEngine {
    pub fn new(sender: Sender<Message>) -> (Self, Producer<Message>) {
        let (incoming_modify_midi_controller, receiver) = Queue::new();
        let message_buffer = RingBuffer::<Message>::new(1000); // TODO verify
        let (mut producer, mut consumer) = message_buffer.split();
        (
            MidiEngine(Arc::new(Mutex::new(MidiEngine_ {
                midi_connections: Vec::with_capacity(MAX_MIDI_CONNECTIONS),
                sender,
                midi_map: MidiMap::new(),
                map_controller: receiver,
                message_buffer: consumer,
            }))),
            producer,
        )
    }
    pub fn connect_all(&self) {
        let connections = connect(self.clone());
        let mut self_ = self.0.lock().unwrap();
        self_.midi_connections = connections;
    }

    fn clone(&self) -> Arc<Mutex<MidiEngine_>> {
        self.0.clone()
    }

    /// Should not be used while playing
    pub fn map_from_file(&self, path: String) {
        todo!();
    }
}

fn connect(midi_engine: Arc<Mutex<MidiEngine_>>) -> Vec<MidiInputConnection<()>> {
    let midi_input = MidiInput::new("main").unwrap();
    let mut midi_connections = vec![];
    for (i, port) in midi_input.ports().iter().enumerate() {
        let midi_input = MidiInput::new("main").unwrap();
        let midi_engine = midi_engine.clone();
        let midi_connection = midi_input
            .connect(
                port,
                &i.to_string(),
                move |timestamp, midi_message, _| {
                    let mut midi_engine = midi_engine.lock().unwrap();
                    midi_engine.handle_message(timestamp, midi_message)
                },
                (),
            )
            .unwrap();
        midi_connections.push(midi_connection);
    }
    midi_connections
}
