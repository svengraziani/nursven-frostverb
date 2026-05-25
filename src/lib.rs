//! Frost Verb DSP/product core.
//!
//! The crate is intentionally split so the realtime DSP engine can be tested
//! independently from the eventual `nice-plug` host wrapper.

pub mod dsp;
pub mod host;
pub mod params;
pub mod preset;

pub use dsp::{FrostVerbEngine, ProcessContext, StereoFrame};
pub use host::{HostParameterSnapshot, PluginDescriptor, StateError};
pub use params::{FrostVerbParams, PARAMETER_DEFS, ParameterDef};
pub use preset::{FACTORY_PRESETS, FactoryPreset, factory_presets_json};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_parameter_ids_are_stable() {
        let ids: Vec<&str> = PARAMETER_DEFS.iter().map(|param| param.id).collect();

        assert_eq!(
            ids,
            vec![
                "coldness",
                "wind",
                "ice_size",
                "ancient_depth",
                "frozen_harmonics",
                "storm",
                "distance",
                "decay",
                "freeze",
                "mix",
                "input",
                "output",
            ]
        );
    }

    #[test]
    fn engine_stays_finite_on_silence() {
        let mut engine = FrostVerbEngine::new();
        engine.prepare(48_000.0);

        let mut params = FrostVerbParams::default();
        params.decay = 1.0;
        params.frozen_harmonics = 1.0;
        params.storm = 1.0;

        for _ in 0..48_000 {
            let out = engine.process_frame(StereoFrame::ZERO, &params);
            assert!(out.left.is_finite());
            assert!(out.right.is_finite());
            assert!(out.left.abs() <= 1.5);
            assert!(out.right.abs() <= 1.5);
        }
    }

    #[test]
    fn params_round_trip_as_stable_array() {
        let params = FrostVerbParams::default();
        let round_trip = FrostVerbParams::from_array(params.as_array());
        assert_eq!(params, round_trip);
    }

    #[test]
    fn all_factory_presets_render_finite_audio() {
        for preset in FACTORY_PRESETS {
            let mut engine = FrostVerbEngine::new();
            engine.prepare(48_000.0);

            for i in 0..12_000 {
                let sample = if i == 0 { 0.5 } else { 0.0 };
                let out = engine.process_frame(
                    StereoFrame {
                        left: sample,
                        right: sample,
                    },
                    &preset.params,
                );
                assert!(out.left.is_finite(), "{}", preset.name);
                assert!(out.right.is_finite(), "{}", preset.name);
            }
        }
    }
}
