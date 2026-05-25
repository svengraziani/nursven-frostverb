use crate::params::FrostVerbParams;

use super::StereoFrame;

const BANDS: usize = 8;
const BAND_HZ: [f32; BANDS] = [146.8, 233.1, 369.9, 587.3, 932.3, 1479.9, 2351.9, 3731.5];

#[derive(Clone, Copy, Debug, Default)]
struct ResonantHold {
    lp: f32,
    held: f32,
    phase: f32,
}

pub struct SpectralFreeze {
    left: [ResonantHold; BANDS],
    right: [ResonantHold; BANDS],
    sample_rate: f32,
    shimmer_phase: f32,
}

impl SpectralFreeze {
    pub fn new() -> Self {
        Self {
            left: [ResonantHold::default(); BANDS],
            right: [ResonantHold::default(); BANDS],
            sample_rate: 48_000.0,
            shimmer_phase: 0.0,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.left = [ResonantHold::default(); BANDS];
        self.right = [ResonantHold::default(); BANDS];
        self.shimmer_phase = 0.0;
    }

    pub fn process(
        &mut self,
        input: StereoFrame,
        amount: f32,
        params: &FrostVerbParams,
    ) -> StereoFrame {
        if amount <= 0.0001 {
            return StereoFrame::ZERO;
        }

        self.shimmer_phase += (0.012 + params.storm * 0.080) / self.sample_rate;
        if self.shimmer_phase > 1.0 {
            self.shimmer_phase -= 1.0;
        }

        let capture = (1.0 - amount).mul_add(0.006, 0.00035);
        let hold_bleed = 0.9993 + amount * 0.00062;
        let harmonic_gain = 0.08 + params.frozen_harmonics * 0.22;
        let cold_tilt = 0.65 + params.coldness * 0.70;
        let mut left = 0.0;
        let mut right = 0.0;

        for band in 0..BANDS {
            let band_weight = 0.65 + band as f32 * 0.075 * cold_tilt;
            let drift = ((self.shimmer_phase + band as f32 * 0.091) * core::f32::consts::TAU).sin()
                * params.storm
                * 0.003;
            let freq = BAND_HZ[band] * (1.0 + drift);
            left += process_band(
                &mut self.left[band],
                input.left,
                freq,
                self.sample_rate,
                capture,
                hold_bleed,
            ) * band_weight;
            right += process_band(
                &mut self.right[band],
                input.right,
                freq * 1.011,
                self.sample_rate,
                capture,
                hold_bleed,
            ) * band_weight;
        }

        StereoFrame {
            left: left * amount * harmonic_gain,
            right: right * amount * harmonic_gain,
        }
        .clamp(0.8)
    }
}

fn process_band(
    band: &mut ResonantHold,
    input: f32,
    freq: f32,
    sample_rate: f32,
    capture: f32,
    hold_bleed: f32,
) -> f32 {
    let coeff = (core::f32::consts::TAU * freq / sample_rate).clamp(0.0001, 0.45);
    band.lp += (input - band.lp) * coeff;
    let bandpassed = input - band.lp;
    band.held = band.held * hold_bleed + bandpassed * capture;
    band.phase += freq / sample_rate;
    if band.phase > 1.0 {
        band.phase -= 1.0;
    }
    band.held * (band.phase * core::f32::consts::TAU).sin()
}
