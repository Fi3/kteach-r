use synthesizer_io_core::module::{Buffer, Module};

pub struct Mixer {}

impl Module for Mixer {
    fn n_bufs_out(&self) -> usize {
        2
    }
    fn process(
        &mut self,
        _control_in: &[f32],
        _control_out: &mut [f32],
        buf_in: &[&Buffer],
        buf_out: &mut [Buffer],
    ) {
        let out = buf_out[0].get_mut();
        for i in 0..out.len() {
            out[i] = 0.0;
        }
        for buf in buf_in {
            let buf = buf.get();
            for i in 0..out.len() {
                out[i] += buf[i];
            }
        }
    }
}
