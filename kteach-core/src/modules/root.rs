use synthesizer_io_core::module::{Buffer, Module};

pub struct Root {}

impl Module for Root {
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
        match buf_in.len() {
            0 => {
                for i in 0..out.len() {
                    out[i] = 0.0;
                }
            }
            _ => {
                let in_ = buf_in[0].get();
                for i in 0..out.len() {
                    out[i] = in_[i];
                }
            }
        }
    }
}
