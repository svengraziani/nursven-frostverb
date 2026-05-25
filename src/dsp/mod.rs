mod conditioning;
mod delay;
mod early_reflections;
mod fdn;
mod freeze;
mod resonance;
mod smooth;
mod wind;

use crate::params::FrostVerbParams;
use conditioning::{DcBlock, SafetyLimiter};
use early_reflections::EarlyReflections;
use fdn::FrozenDiffusionNetwork;
use freeze::SpectralFreeze;
use resonance::IceResonanceLayer;
use smooth::SmoothedValue;
use wind::WindLayer;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StereoFrame {
    pub left: f32,
    pub right: f32,
}

impl StereoFrame {
    pub const ZERO: Self = Self {
        left: 0.0,
        right: 0.0,
    };

    pub fn mono(self) -> f32 {
        (self.left + self.right) * 0.5
    }

    pub fn clamp(self, ceiling: f32) -> Self {
        Self {
            left: self.left.clamp(-ceiling, ceiling),
            right: self.right.clamp(-ceiling, ceiling),
        }
    }
}

impl core::ops::Add for StereoFrame {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            left: self.left + rhs.left,
            right: self.right + rhs.right,
        }
    }
}

impl core::ops::Mul<f32> for StereoFrame {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            left: self.left * rhs,
            right: self.right * rhs,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EngineMeters {
    pub input_peak: f32,
    pub output_peak: f32,
    pub wet_peak: f32,
    pub limiter_gain_reduction_db: f32,
}

impl EngineMeters {
    fn observe_input(&mut self, frame: StereoFrame) {
        self.input_peak = self.input_peak.max(frame.left.abs()).max(frame.right.abs());
    }

    fn observe_wet(&mut self, frame: StereoFrame) {
        self.wet_peak = self.wet_peak.max(frame.left.abs()).max(frame.right.abs());
    }

    fn observe_output(&mut self, frame: StereoFrame) {
        self.output_peak = self
            .output_peak
            .max(frame.left.abs())
            .max(frame.right.abs());
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ProcessContext {
    pub sample_rate: f32,
    pub block_size: usize,
}

impl Default for ProcessContext {
    fn default() -> Self {
        Self {
            sample_rate: 48_000.0,
            block_size: 64,
        }
    }
}

pub struct FrostVerbEngine {
    sample_rate: f32,
    input_gain: SmoothedValue,
    output_gain: SmoothedValue,
    mix: SmoothedValue,
    freeze_amount: SmoothedValue,
    input_dc_l: DcBlock,
    input_dc_r: DcBlock,
    output_dc_l: DcBlock,
    output_dc_r: DcBlock,
    early: EarlyReflections,
    fdn: FrozenDiffusionNetwork,
    resonance: IceResonanceLayer,
    wind: WindLayer,
    freeze: SpectralFreeze,
    limiter_l: SafetyLimiter,
    limiter_r: SafetyLimiter,
    meters: EngineMeters,
}

impl FrostVerbEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            sample_rate: 48_000.0,
            input_gain: SmoothedValue::new(0.75),
            output_gain: SmoothedValue::new(0.75),
            mix: SmoothedValue::new(0.34),
            freeze_amount: SmoothedValue::new(0.0),
            input_dc_l: DcBlock::default(),
            input_dc_r: DcBlock::default(),
            output_dc_l: DcBlock::default(),
            output_dc_r: DcBlock::default(),
            early: EarlyReflections::new(),
            fdn: FrozenDiffusionNetwork::new(),
            resonance: IceResonanceLayer::new(),
            wind: WindLayer::new(),
            freeze: SpectralFreeze::new(),
            limiter_l: SafetyLimiter::new(0.98),
            limiter_r: SafetyLimiter::new(0.98),
            meters: EngineMeters::default(),
        };
        engine.prepare(48_000.0);
        engine
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate.max(8_000.0);
        self.input_gain.set_smoothing_time(self.sample_rate, 0.015);
        self.output_gain.set_smoothing_time(self.sample_rate, 0.015);
        self.mix.set_smoothing_time(self.sample_rate, 0.030);
        self.freeze_amount
            .set_smoothing_time(self.sample_rate, 0.080);
        self.early.prepare(self.sample_rate);
        self.fdn.prepare(self.sample_rate);
        self.resonance.prepare(self.sample_rate);
        self.wind.prepare(self.sample_rate);
        self.freeze.prepare(self.sample_rate);
        self.limiter_l.prepare(self.sample_rate);
        self.limiter_r.prepare(self.sample_rate);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.input_gain.reset(0.75);
        self.output_gain.reset(0.75);
        self.mix.reset(0.34);
        self.freeze_amount.reset(0.0);
        self.input_dc_l = DcBlock::default();
        self.input_dc_r = DcBlock::default();
        self.output_dc_l = DcBlock::default();
        self.output_dc_r = DcBlock::default();
        self.early.reset();
        self.fdn.reset();
        self.resonance.reset();
        self.wind.reset();
        self.freeze.reset();
        self.limiter_l.reset();
        self.limiter_r.reset();
        self.meters = EngineMeters::default();
    }

    pub fn process_frame(&mut self, input: StereoFrame, params: &FrostVerbParams) -> StereoFrame {
        let params = params.sanitized();
        self.input_gain
            .set_target(db_gain(params.input, -24.0, 12.0));
        self.output_gain
            .set_target(db_gain(params.output, -24.0, 12.0));
        self.mix.set_target(params.mix);
        self.freeze_amount.set_target(params.freeze);

        let input_gain = self.input_gain.next();
        let output_gain = self.output_gain.next();
        let mix = self.mix.next();
        let freeze = self.freeze_amount.next();

        let dry = StereoFrame {
            left: self.input_dc_l.process(input.left * input_gain),
            right: self.input_dc_r.process(input.right * input_gain),
        };
        self.meters.observe_input(dry);

        let early = self.early.process(dry, &params);
        let body = self.fdn.process(dry + early, &params);
        let resonance = self.resonance.process(body, &params);
        let wind = self.wind.process(dry, &params);
        let frozen = self.freeze.process(body + resonance, freeze, &params);
        let wet = early + body + resonance + wind + frozen;
        self.meters.observe_wet(wet);

        let out = dry * (1.0 - mix) + wet * mix;
        let limited = StereoFrame {
            left: self
                .limiter_l
                .process(self.output_dc_l.process(out.left * output_gain)),
            right: self
                .limiter_r
                .process(self.output_dc_r.process(out.right * output_gain)),
        };
        self.meters.limiter_gain_reduction_db = self
            .limiter_l
            .gain_reduction_db()
            .min(self.limiter_r.gain_reduction_db());
        self.meters.observe_output(limited);
        limited.clamp(1.0)
    }

    pub fn process_block(
        &mut self,
        input: &[StereoFrame],
        output: &mut [StereoFrame],
        params: &FrostVerbParams,
    ) {
        assert_eq!(input.len(), output.len());
        self.meters = EngineMeters::default();
        for (input, output) in input.iter().zip(output.iter_mut()) {
            *output = self.process_frame(*input, params);
        }
    }

    pub fn process_interleaved_in_place(&mut self, buffer: &mut [f32], params: &FrostVerbParams) {
        assert_eq!(buffer.len() % 2, 0);
        self.meters = EngineMeters::default();
        for frame in buffer.chunks_exact_mut(2) {
            let out = self.process_frame(
                StereoFrame {
                    left: frame[0],
                    right: frame[1],
                },
                params,
            );
            frame[0] = out.left;
            frame[1] = out.right;
        }
    }

    pub fn meters(&self) -> EngineMeters {
        self.meters
    }
}

impl Default for FrostVerbEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn db_gain(normalized: f32, min_db: f32, max_db: f32) -> f32 {
    let db = min_db + (max_db - min_db) * normalized;
    10.0_f32.powf(db / 20.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_tail_decays_and_stays_bounded() {
        let mut engine = FrostVerbEngine::new();
        engine.prepare(48_000.0);
        let mut params = FrostVerbParams::default();
        params.decay = 0.92;
        params.ice_size = 0.8;
        params.mix = 1.0;

        let mut max = 0.0_f32;
        for i in 0..96_000 {
            let input = if i == 0 {
                StereoFrame {
                    left: 1.0,
                    right: 1.0,
                }
            } else {
                StereoFrame::ZERO
            };
            let out = engine.process_frame(input, &params);
            max = max.max(out.left.abs()).max(out.right.abs());
            assert!(out.left.is_finite());
            assert!(out.right.is_finite());
        }

        assert!(max <= 1.0);
    }

    #[test]
    fn interleaved_block_processing_matches_frame_count() {
        let mut engine = FrostVerbEngine::new();
        engine.prepare(48_000.0);
        let params = FrostVerbParams::default();
        let mut buffer = vec![0.0; 512];
        buffer[0] = 0.5;
        buffer[1] = -0.25;

        engine.process_interleaved_in_place(&mut buffer, &params);

        assert_eq!(buffer.len(), 512);
        assert!(buffer.iter().all(|sample| sample.is_finite()));
        assert!(engine.meters().input_peak > 0.0);
    }

    #[test]
    fn reset_is_deterministic() {
        let mut engine = FrostVerbEngine::new();
        engine.prepare(48_000.0);
        let mut params = FrostVerbParams::default();
        params.wind = 0.6;
        params.storm = 0.7;

        let first = engine.process_frame(
            StereoFrame {
                left: 0.25,
                right: -0.10,
            },
            &params,
        );

        engine.reset();
        let second = engine.process_frame(
            StereoFrame {
                left: 0.25,
                right: -0.10,
            },
            &params,
        );

        assert_eq!(first, second);
    }

    #[test]
    fn extreme_settings_soak_stays_finite_and_limited() {
        let mut engine = FrostVerbEngine::new();
        engine.prepare(96_000.0);
        let params = FrostVerbParams {
            coldness: 1.0,
            wind: 1.0,
            ice_size: 1.0,
            ancient_depth: 1.0,
            frozen_harmonics: 1.0,
            storm: 1.0,
            distance: 1.0,
            decay: 1.0,
            freeze: 1.0,
            mix: 1.0,
            input: 1.0,
            output: 1.0,
        };

        for i in 0..96_000 {
            let sample = if i % 997 == 0 { 1.0 } else { 0.0 };
            let out = engine.process_frame(
                StereoFrame {
                    left: sample,
                    right: -sample,
                },
                &params,
            );
            assert!(out.left.is_finite());
            assert!(out.right.is_finite());
            assert!(out.left.abs() <= 1.0);
            assert!(out.right.abs() <= 1.0);
        }
    }
}
