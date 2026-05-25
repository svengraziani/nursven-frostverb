use crate::params::FrostVerbParams;

use super::StereoFrame;

const MODES: [f32; 5] = [521.0, 883.0, 1321.0, 2107.0, 3191.0];

#[derive(Clone, Copy, Debug, Default)]
struct Bandpass {
    z1: f32,
    z2: f32,
    b0: f32,
    b2: f32,
    a1: f32,
    a2: f32,
}

impl Bandpass {
    fn set(&mut self, sample_rate: f32, freq: f32, q: f32) {
        let omega = core::f32::consts::TAU * (freq / sample_rate).clamp(0.001, 0.45);
        let alpha = omega.sin() / (2.0 * q.max(0.2));
        let cos = omega.cos();
        let a0 = 1.0 + alpha;
        self.b0 = alpha / a0;
        self.b2 = -alpha / a0;
        self.a1 = -2.0 * cos / a0;
        self.a2 = (1.0 - alpha) / a0;
    }

    fn process(&mut self, input: f32) -> f32 {
        let out = input * self.b0 + self.z1;
        self.z1 = self.z2 - self.a1 * out;
        self.z2 = input * self.b2 - self.a2 * out;
        out
    }
}

pub struct IceResonanceLayer {
    left: [Bandpass; 5],
    right: [Bandpass; 5],
    sample_rate: f32,
}

impl IceResonanceLayer {
    pub fn new() -> Self {
        Self {
            left: [Bandpass::default(); 5],
            right: [Bandpass::default(); 5],
            sample_rate: 48_000.0,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.reset();
        self.configure(0.5, 0.2);
    }

    pub fn reset(&mut self) {
        self.left = [Bandpass::default(); 5];
        self.right = [Bandpass::default(); 5];
    }

    pub fn process(&mut self, input: StereoFrame, params: &FrostVerbParams) -> StereoFrame {
        self.configure(params.coldness, params.storm);
        let amount = params.frozen_harmonics * (0.12 + params.coldness * 0.23);
        let q = 0.65 + params.frozen_harmonics * 0.35;
        let mut left = 0.0;
        let mut right = 0.0;

        for mode in 0..5 {
            left += self.left[mode].process(input.left) * q;
            right += self.right[mode].process(input.right) * q;
        }

        StereoFrame {
            left: left * amount,
            right: right * amount,
        }
    }

    fn configure(&mut self, coldness: f32, storm: f32) {
        for (index, base) in MODES.iter().enumerate() {
            let spread = 1.0 + coldness * 0.18 + storm * 0.03 * index as f32;
            let q = 2.0 + coldness * 7.0;
            self.left[index].set(self.sample_rate, base * spread, q);
            self.right[index].set(self.sample_rate, base * (spread + 0.017), q * 0.92);
        }
    }
}
