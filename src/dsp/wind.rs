use crate::params::FrostVerbParams;

use super::StereoFrame;

const SEED_L: u32 = 0x5EED_1234;
const SEED_R: u32 = 0xF005_7B1D;

pub struct WindLayer {
    sample_rate: f32,
    seed_l: u32,
    seed_r: u32,
    lp_l: f32,
    lp_r: f32,
    phase: f32,
}

impl WindLayer {
    pub fn new() -> Self {
        Self {
            sample_rate: 48_000.0,
            seed_l: SEED_L,
            seed_r: SEED_R,
            lp_l: 0.0,
            lp_r: 0.0,
            phase: 0.0,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.seed_l = SEED_L;
        self.seed_r = SEED_R;
        self.lp_l = 0.0;
        self.lp_r = 0.0;
        self.phase = 0.0;
    }

    pub fn process(&mut self, input: StereoFrame, params: &FrostVerbParams) -> StereoFrame {
        self.phase += (0.04 + params.storm * 0.16) / self.sample_rate;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        let movement = (core::f32::consts::TAU * self.phase).sin() * 0.5 + 0.5;
        let envelope = (1.0 - input.mono().abs().min(1.0)).sqrt();
        let amount = params.wind * (0.02 + params.storm * 0.05) * envelope;
        let coeff_l = 0.002 + movement * 0.018 + params.distance * 0.012;
        let coeff_r = 0.002 + (1.0 - movement) * 0.018 + params.distance * 0.012;

        let noise_l = white(&mut self.seed_l);
        let noise_r = white(&mut self.seed_r);
        self.lp_l += (noise_l - self.lp_l) * coeff_l;
        self.lp_r += (noise_r - self.lp_r) * coeff_r;

        StereoFrame {
            left: self.lp_l * amount,
            right: self.lp_r * amount,
        }
    }
}

fn white(seed: &mut u32) -> f32 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;
    (x as f32 / u32::MAX as f32) * 2.0 - 1.0
}
