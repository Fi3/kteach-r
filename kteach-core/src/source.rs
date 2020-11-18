//! ...
//!

pub trait Source {
    fn next_sample_buffer(&mut self) -> Vec<f32>;
}
