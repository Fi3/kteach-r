//! Thread and alloc safe struct containing the app state
//!
//!
use crate::player::Player;
use std::collections::HashMap;
use std::convert::From;
use synthesizer_io_core::graph::Graph;
use synthesizer_io_core::queue::{Item, Queue, Receiver, Sender};

pub type Track_ = Vec<Vec<f32>>;
