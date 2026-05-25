use crate::params::FrostVerbParams;

use super::{StereoFrame, delay::DelayLine};

const TAP_MS: [f32; 6] = [7.3, 11.9, 17.1, 23.8, 31.4, 43.7];
const TAP_GAIN: [f32; 6] = [0.42, -0.35, 0.30, -0.25, 0.22, -0.18];

pub struct EarlyReflections {
    left: DelayLine,
    right: DelayLine,
    sample_rate: f32,
}

impl EarlyReflections {
    pub fn new() -> Self {
        Self {
            left: DelayLine::new(),
            right: DelayLine::new(),
            sample_rate: 48_000.0,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        let max_delay = (sample_rate * 0.090) as usize;
        self.left.resize(max_delay);
        self.right.resize(max_delay);
    }

    pub fn reset(&mut self) {
        self.left.clear();
        self.right.clear();
    }

    pub fn process(&mut self, input: StereoFrame, params: &FrostVerbParams) -> StereoFrame {
        let scale = 0.55 + params.ice_size * 1.7 + params.distance * 0.5;
        let brightness = 0.55 + params.coldness * 0.45;
        let mut left = 0.0;
        let mut right = 0.0;

        for (index, ms) in TAP_MS.iter().enumerate() {
            let delay_l = ((*ms * scale) * self.sample_rate / 1000.0) as usize;
            let delay_r = (((*ms + 3.1) * scale) * self.sample_rate / 1000.0) as usize;
            let gain = TAP_GAIN[index] * brightness * (1.0 - 0.35 * params.distance);
            left += self.left.push_read(input.left, delay_l) * gain;
            right += self.right.push_read(input.right, delay_r) * -gain;
        }

        StereoFrame { left, right }
    }
}
