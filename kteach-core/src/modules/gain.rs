use synthesizer_io_core::module::{Buffer, Module};

pub struct Gain {
    gain: f32,
}

impl Gain {
    pub fn new() -> Gain {
        Gain { gain: 1.0 }
    }
}

impl Module for Gain {
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
        let buf = buf_in[0].get();
        for i in 0..out.len() {
            out[i] = buf[i] * self.gain;
        }
    }

    fn set_param(&mut self, _param_ix: usize, val: f32, _timestamp: u64) {
        let normalized = val / 127.0;
        self.gain = normalized;
    }
}
