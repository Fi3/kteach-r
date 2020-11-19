//! ..
//!
use synthesizer_io_core::module::{Buffer, Module, N_SAMPLES_PER_CHUNK};

pub type Track = Vec<[f32; N_SAMPLES_PER_CHUNK]>;

pub enum PlayerState {
    Play,
    Pause,
}

pub struct Player {
    track: Track,
    state: PlayerState,
    cursor: usize,
}

impl Module for Player {
    fn n_bufs_out(&self) -> usize {
        2
    }

    fn process(
        &mut self,
        _control_in: &[f32],
        _control_out: &mut [f32],
        _buf_in: &[&Buffer],
        buf_out: &mut [Buffer],
    ) {
        let out = buf_out[0].get_mut();
        match &self.state {
            PlayerState::Pause => {
                for i in 0..out.len() {
                    out[i] = 0.0;
                }
            }
            PlayerState::Play => {
                if self.cursor == (self.track.len() - 1) {
                    self.cursor = 0
                };
                let next = self.track[self.cursor];
                self.cursor = self.cursor + 1;
                for i in 0..N_SAMPLES_PER_CHUNK {
                    out[i] = next[i];
                }
            }
        }
    }

    fn set_param(&mut self, param_ix: usize, _val: f32, _timestamp: u64) {
        match param_ix {
            0 => self.state = PlayerState::Play,
            1 => self.state = PlayerState::Pause,
            _ => (),
        }
    }
}

impl Player {
    pub fn new(track: Track, cursor: Option<usize>, state: Option<PlayerState>) -> Self {
        let cursor = cursor.unwrap_or(0);
        let state = state.unwrap_or(PlayerState::Pause);
        Player {
            track,
            state,
            cursor,
        }
    }
}

//impl Player {
//    pub fn change_track(&mut self, track: &'track Option<Track_>) -> &'track Option<Track_> {
//        match self.state {
//            PlayerState::Play => &None,
//            PlayerState::Pause => {
//                let old_track = self.track;
//                self.cursor = 0;
//                old_track
//            }
//        }
//    }
//
//    pub fn change_state(&mut self, state: PlayerState) {
//        match self.track {
//            None => (),
//            Some(_) => self.state = state,
//        }
//    }
//
//    pub fn is_paused(&self) -> bool {
//        match self.state {
//            PlayerState::Pause => true,
//            PlayerState::Play => false,
//        }
//    }
//
//    pub fn play(&mut self) {
//        self.state = PlayerState::Play;
//    }
//
//    pub fn pause(&mut self) {
//        self.state = PlayerState::Pause;
//    }
//}

//impl<'track> Source for Player<'track> {
//    fn next_sample_buffer(&mut self) -> Vec<f32> {
//        match &self.track {
//            None => vec![0.0, 0.0],
//            Some(track) => {
//                if self.cursor == track.len() - 1 {
//                    self.cursor = 0
//                };
//                let next = track[self.cursor].clone();
//                self.cursor = self.cursor + 1;
//                next
//            }
//        }
//    }
//}
