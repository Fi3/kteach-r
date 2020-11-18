const MAX_PLAYERS: usize = 100;

use crate::player::Player;
use std::collections::HashMap;
use std::convert::From;
use synthesizer_io_core::queue::{Item, Queue, Receiver, Sender};

pub type Track_ = Vec<Vec<f32>>;
pub type Players_<'track> = HashMap<u16, Player<'track>>;

pub struct State<'track> {
    players: Players_<'track>,
    receive_from_control: Receiver<Message>,
    free_memory: Sender<Item<Message>>,
}

pub enum Message {
    /// Create a new paused player with no track (Player, player_id)
    //AddPlayer(Player, u16),
    /// Change a track (track, player_id)
    /// It works only if the player is paused
    /// If the player do not have a track just add the track
    ChangeTrack(Option<Track_>, u16),
    /// It remove a player (player_id)
    /// It works only if the player is paused
    //RemovePlayer(u16),
    /// Trigger play for player_id (player_id)
    PlayPlayer(u16),
    /// Trigger pause for player_id (player_id)
    PausePlayer(u16),
}

//impl From<Message> for Item<Message> {
//    fn from(inner: Message) -> Self {
//        Item::make_item(inner)
//    }
//}

impl<'track> State<'track> {
    pub fn initialize() -> (Self, Sender<Message>, Receiver<Item<Message>>) {
        let (sender, receiver) = Queue::new();
        let (free_memory_sender, free_memory_receiver) = Queue::new();
        let initial_state = State {
            players: HashMap::with_capacity(MAX_PLAYERS), // TODO check that is allocation free also for removing values
            receive_from_control: receiver,
            free_memory: free_memory_sender,
        };
        (initial_state, sender, free_memory_receiver)
    }

    pub fn send_message(sender: Sender<Message>, message: Message) {
        sender.send(message);
    }

    pub fn update(&mut self, receiver: Receiver<Message>) {
        let messages = receiver.recv_items();
        for message in messages {
            match &*message {
                //Message::AddPlayer(player, player_id) => self.add_player(player, *player_id),
                Message::ChangeTrack(track, player_id) => {
                    self.change_track(track, *player_id, message)
                }
                //Message::RemovePlayer(player_id) => self.remove_player(*player_id),
                Message::PlayPlayer(player_id) => self.play_player(*player_id),
                Message::PausePlayer(player_id) => self.pause_player(*player_id),
            }
        }
    }

    fn add_player(&mut self, player: &mut Player, player_id: u16) {
        //self.players.insert(player_id, *x);
    }

    fn change_track(
        &mut self,
        track: &'track Option<Track_>,
        player_id: u16,
        message: Item<Message>,
    ) {
        let player = self.players.get_mut(&player_id).unwrap();
        if player.is_paused() {
            let swapped = player.change_track(track);
            match swapped {
                None => (),
                Some(_) => self.free_memory.send(message),
            }
        }
    }

    fn remove_player(&mut self, player_id: u16) {
        // let player = self.players.remove(&player_id).unwrap();
        // if player.is_paused() {
        //     self.free_memory
        //         .send(Item::make_item(Message::AddPlayer(player, player_id)));
        // } else {
        //     self.players.insert(player_id, player);
        // };
    }

    fn play_player(&mut self, player_id: u16) {
        let player = self.players.get_mut(&player_id).unwrap();
        player.play();
    }

    fn pause_player(&mut self, player_id: u16) {
        let player = self.players.get_mut(&player_id).unwrap();
        player.pause();
    }
}

//// Fare una struct che si occupa di creare e tenere traccia di id unici
