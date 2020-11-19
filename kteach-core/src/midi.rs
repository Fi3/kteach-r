use midir::{MidiInput, MidiInputConnection};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::convert::TryFrom;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use synthesizer_io_core::graph::{Message as Message_, SetParam};
use synthesizer_io_core::queue::{Item, Queue, Receiver, Sender};
use wmidi::{
    Channel, ControlFunction, ControlValue, MidiMessage as MidiMessage_, Note, ProgramNumber,
    Velocity,
};

pub use midir;
pub use ringbuf;
pub use wmidi;

// arbitraly chosen
const MIDI_MAP_CAPACITY: usize = 10;
const MAX_MIDI_CONNECTIONS: usize = 10;

const NUMBER_OF_MIDI_CHANNELS: usize = 16;
const NUMBER_OF_MIDI_OPTIONAL_INDEXES: usize = 128; // 2^7 (control functions are 7 bit)

/// Number of possible map for supported messages is:
///
/// note_off = channels * notes                      21376 +
/// note_on = channels * notes                       21376 +
/// polyphonic_key_pressure = channels * notes       21376 +
/// control_change = channels * control_functions    21376 +
/// program_change = channels * program_numbers      21376 +
/// channel_pressure = channels                         16 +
///                                                  -------
///                                                  106896
///
/// For now everything is preallocated then probably a better strategy can be used.
///
const NUMBER_OF_TEMPLATES: usize = 106896;
const NOTE_OFF_INDEX: usize = 0 * NUMBER_OF_MIDI_CHANNELS * NUMBER_OF_MIDI_OPTIONAL_INDEXES;
const NOTE_ON_INDEX: usize = 1 * NUMBER_OF_MIDI_CHANNELS * NUMBER_OF_MIDI_OPTIONAL_INDEXES;
const PK_PRESSURE_INDEX: usize = 2 * NUMBER_OF_MIDI_CHANNELS * NUMBER_OF_MIDI_OPTIONAL_INDEXES;
const CONTROL_CHANGE_INDEX: usize = 3 * NUMBER_OF_MIDI_CHANNELS * NUMBER_OF_MIDI_OPTIONAL_INDEXES;
const PROGRAM_CHANGE_INDEX: usize = 4 * NUMBER_OF_MIDI_CHANNELS * NUMBER_OF_MIDI_OPTIONAL_INDEXES;
const CHANNEL_PRESSURE_INDEX: usize = 5 * NUMBER_OF_MIDI_CHANNELS * NUMBER_OF_MIDI_OPTIONAL_INDEXES;

type Message = Item<Message_>;

/// kteach midi missage is a subset of wmidi::MidiMessage
pub enum MidiMessage {
    NoteOff(Channel, Note, Velocity),
    NoteOn(Channel, Note, Velocity),
    PolyphonicKeyPressure(Channel, Note, Velocity),
    ControlChange(Channel, ControlFunction, ControlValue),
    ProgramChange(Channel, ProgramNumber),
    ChannelPressure(Channel, Velocity),
}

impl<'a> From<MidiMessage_<'a>> for MidiMessage {
    fn from(wmidi_message: MidiMessage_) -> Self {
        match wmidi_message {
            MidiMessage_::NoteOff(channel, note, velocity) => {
                MidiMessage::NoteOff(channel, note, velocity)
            }
            MidiMessage_::NoteOn(channel, note, velocity) => {
                MidiMessage::NoteOn(channel, note, velocity)
            }
            MidiMessage_::PolyphonicKeyPressure(channel, note, velocity) => {
                MidiMessage::PolyphonicKeyPressure(channel, note, velocity)
            }
            MidiMessage_::ControlChange(channel, control_function, control_value) => {
                MidiMessage::ControlChange(channel, control_function, control_value)
            }
            MidiMessage_::ProgramChange(channel, program_number) => {
                MidiMessage::ProgramChange(channel, program_number)
            }
            MidiMessage_::ChannelPressure(channel, velocity) => {
                MidiMessage::ChannelPressure(channel, velocity)
            }
            _ => panic!(), // unsupported message
        }
    }
}

struct MidiMap {
    map: Vec<Vec<SetParam>>,
}

impl MidiMap {
    pub fn new() -> Self {
        let mut map = Vec::with_capacity(NUMBER_OF_TEMPLATES);
        for _ in 0..NUMBER_OF_TEMPLATES {
            let void_param_templates = Vec::with_capacity(MIDI_MAP_CAPACITY);
            map.push(void_param_templates);
        }
        MidiMap { map }
    }

    pub fn get(&mut self, midi_message: MidiMessage) -> (&Vec<SetParam>, Option<u8>) {
        let (index, value) = midimap_index(midi_message);
        let templates = &self.map[index];
        (templates, value)
    }

    pub fn set(&mut self, template: SetParam, midi_message: MidiMessage) {
        let (index, _) = midimap_index(midi_message);
        let templates = &mut self.map[index];
        // NUMBER_OF_TEMPLATES not enforced TODO check len maybe
        templates.push(template);
    }
}

struct MidiEngine_ {
    midi_connections: Vec<MidiInputConnection<()>>,
    sender: Sender<Message_>,
    midi_map: MidiMap,
    // TODO it use a ringbuffer because of the easy API, maybe use a synthesizer_io_core::Quee
    map_controller: Consumer<(SetParam, MidiMessage)>,
    // TODO it use a ringbuffer because of the easy API, maybe use a synthesizer_io_core::Quee
    message_buffer: Consumer<Message>,
}

impl MidiEngine_ {
    // Receive the raw midi message,
    // Update the midimap
    // parse the recived message in a MidiMessage
    // get the registered templates from midimap (they just indicate which Module the message is targeting)
    // try to get an already allocated Item<Message> from the ring buffer.
    //   If not present continue with the next template.
    //   If it is present modify it with the new values and then send it to the Worker
    #[inline]
    fn handle_midi_message(&mut self, _timestamp_micro_seconds: u64, raw_midi_message: &[u8]) {
        self.update_midi_map();
        let midi_message = MidiMessage_::try_from(raw_midi_message).unwrap().into();
        let (templates, value) = self.midi_map.get(midi_message);
        for template in templates {
            let to_worker_message = self.message_buffer.pop();
            match to_worker_message {
                None => continue,
                Some(message) => {
                    let message = process_message(template, message, value.unwrap_or(0));
                    self.sender.send_item(message)
                }
            }
        }
    }

    #[inline]
    fn update_midi_map(&mut self) {
        let mut new_map = self.map_controller.pop();
        while new_map.is_some() {
            let (template, midi_message) = new_map.unwrap();
            self.midi_map.set(template, midi_message);
            new_map = self.map_controller.pop();
        }
    }
}

pub struct MidiEngine(Arc<Mutex<MidiEngine_>>);

impl MidiEngine {
    pub fn new(
        sender: Sender<Message_>,
    ) -> (Self, Producer<Message>, Producer<(SetParam, MidiMessage)>) {
        let message_buffer = RingBuffer::<Message>::new(1000); // TODO verify
        let update_map_buffer = RingBuffer::<(SetParam, MidiMessage)>::new(50); // TODO verify
        let (mut message_producer, message_consumer) = message_buffer.split();
        let (update_map_producer, update_map_consumer) = update_map_buffer.split();
        // Pre allocate message_buffer
        for _ in 0..1000 {
            let _ = message_producer.push(Item::make_item(Message_::SetParam(SetParam {
                ix: 0,
                param_ix: 0,
                val: 0.0,
                timestamp: 0,
            })));
        }
        (
            MidiEngine(Arc::new(Mutex::new(MidiEngine_ {
                midi_connections: Vec::with_capacity(MAX_MIDI_CONNECTIONS),
                sender,
                midi_map: MidiMap::new(),
                map_controller: update_map_consumer,
                message_buffer: message_consumer,
            }))),
            message_producer,
            update_map_producer,
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
    #[allow(unused_variables)]
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
        let midi_connection = midi_input.connect(
            port,
            &i.to_string(),
            move |timestamp, midi_message, _| {
                let mut midi_engine = midi_engine.lock().unwrap();
                midi_engine.handle_midi_message(timestamp, midi_message)
            },
            (),
        );
        if midi_connection.is_ok() {
            midi_connections.push(midi_connection.unwrap());
        }
    }
    if midi_connections.len() == 0 {
        panic!("No midi port avaiable");
    }
    midi_connections
}

// TODO test it!!
#[inline]
fn midimap_index(message: MidiMessage) -> (usize, Option<u8>) {
    match message {
        MidiMessage::NoteOff(channel, note, velocity) => {
            let optional_index: u8 = note.into();
            (
                NOTE_OFF_INDEX + innner_midi_index(channel.number(), optional_index + 1) as usize,
                Some(velocity.into()),
            )
        }
        MidiMessage::NoteOn(channel, note, velocity) => {
            let optional_index: u8 = note.into();
            (
                NOTE_ON_INDEX + innner_midi_index(channel.number(), optional_index + 1) as usize,
                Some(velocity.into()),
            )
        }
        MidiMessage::PolyphonicKeyPressure(channel, note, velocity) => {
            let optional_index: u8 = note.into();
            (
                PK_PRESSURE_INDEX
                    + innner_midi_index(channel.number(), optional_index + 1) as usize,
                Some(velocity.into()),
            )
        }
        MidiMessage::ControlChange(channel, control_function, control_value) => {
            let optional_index: u8 = control_function.into();
            (
                CONTROL_CHANGE_INDEX
                    + innner_midi_index(channel.number(), optional_index + 1) as usize,
                Some(control_value.into()),
            )
        }
        MidiMessage::ProgramChange(channel, program_number) => {
            let optional_index: u8 = program_number.into();
            (
                PROGRAM_CHANGE_INDEX
                    + innner_midi_index(channel.number(), optional_index + 1) as usize,
                None,
            )
        }
        MidiMessage::ChannelPressure(channel, velocity) => (
            CHANNEL_PRESSURE_INDEX + innner_midi_index(channel.number(), 1) as usize,
            Some(velocity.into()),
        ),
        _ => panic!(),
    }
}

#[inline]
fn innner_midi_index(channel: u8, optional_index: u8) -> u16 {
    debug_assert!(channel <= NUMBER_OF_MIDI_CHANNELS as u8);
    debug_assert!(optional_index <= NUMBER_OF_MIDI_OPTIONAL_INDEXES as u8);
    channel as u16 * optional_index as u16
}

// Take message template from the MidiMap
// And an already allocated Item<Message_> from the buffer
#[inline]
fn process_message<T: Into<f32> + Copy>(
    message_template: &SetParam,
    mut to_worker_message: Message,
    val: T,
) -> Message {
    match (message_template, to_worker_message.deref_mut()) {
        (set_param_template, Message_::SetParam(set_param)) => {
            set_param.ix = set_param_template.ix;
            set_param.param_ix = set_param_template.param_ix;
            set_param.val = val.into();
            set_param.timestamp = 0;
        }
        _ => panic!(), // this should be an impossible state
    }
    to_worker_message
}
