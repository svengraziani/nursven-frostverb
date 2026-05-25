use crate::params::FrostVerbParams;

use super::{
    StereoFrame,
    conditioning::{DcBlock, softclip},
    delay::DelayLine,
};

const LANES: usize = 8;
const BASE_MS: [f32; LANES] = [41.1, 53.7, 67.9, 79.3, 91.7, 103.1, 127.9, 149.3];

pub struct FrozenDiffusionNetwork {
    delays: [DelayLine; LANES],
    feedback_state: [f32; LANES],
    dc: [DcBlock; LANES],
    sample_rate: f32,
    phase: f32,
}

impl FrozenDiffusionNetwork {
    pub fn new() -> Self {
        Self {
            delays: core::array::from_fn(|_| DelayLine::new()),
            feedback_state: [0.0; LANES],
            dc: [DcBlock::default(); LANES],
            sample_rate: 48_000.0,
            phase: 0.0,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        for (delay, ms) in self.delays.iter_mut().zip(BASE_MS) {
            delay.resize((sample_rate * (ms + 30.0) / 1000.0) as usize);
        }
        self.reset();
    }

    pub fn reset(&mut self) {
        self.feedback_state = [0.0; LANES];
        self.phase = 0.0;
        for delay in &mut self.delays {
            delay.clear();
        }
        for dc in &mut self.dc {
            *dc = DcBlock::default();
        }
    }

    pub fn process(&mut self, input: StereoFrame, params: &FrostVerbParams) -> StereoFrame {
        self.phase += 0.00013 + params.storm * 0.00037;
        if self.phase > core::f32::consts::TAU {
            self.phase -= core::f32::consts::TAU;
        }

        let feedback = (0.50 + params.decay * 0.42 + params.ancient_depth * 0.04).min(0.965);
        let damping = 0.18 + (1.0 - params.coldness) * 0.22;
        let mono = input.mono() * (0.25 + params.distance * 0.15);

        let mixed = householder(self.feedback_state);
        let mut lane_out = [0.0; LANES];

        for lane in 0..LANES {
            let drift = (self.phase + lane as f32 * 0.73).sin() * params.storm * 0.0015;
            let delay_samples = ((BASE_MS[lane] * (0.55 + params.ice_size * 1.3 + drift))
                * self.sample_rate
                / 1000.0) as usize;
            let injected = mono * if lane % 2 == 0 { 1.0 } else { -1.0 };
            let fb = self.dc[lane].process(softclip(mixed[lane] * feedback));
            lane_out[lane] = self.delays[lane].push_read(injected + fb, delay_samples);
            self.feedback_state[lane] =
                lane_out[lane] * (1.0 - damping) + self.feedback_state[lane] * damping;
        }

        let left = lane_out[0] + lane_out[2] - lane_out[5] + lane_out[7];
        let right = lane_out[1] - lane_out[3] + lane_out[4] + lane_out[6];
        StereoFrame {
            left: left * 0.18,
            right: right * 0.18,
        }
    }
}

fn householder(input: [f32; LANES]) -> [f32; LANES] {
    let sum = input.iter().sum::<f32>() * (2.0 / LANES as f32);
    let mut output = [0.0; LANES];
    for lane in 0..LANES {
        output[lane] = sum - input[lane];
    }
    output
}
