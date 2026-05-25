//! C ABI for native plugin wrappers.
//!
//! The JUCE wrapper uses this thin boundary to keep the Rust DSP core as the
//! single source of truth while the plugin shell, parameters, and WebView UI
//! live in C++.

use crate::{
    dsp::{EngineMeters, FrostVerbEngine},
    params::{FrostVerbParams, PARAMETER_DEFS},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FrostVerbFfiParams {
    pub coldness: f32,
    pub wind: f32,
    pub ice_size: f32,
    pub ancient_depth: f32,
    pub frozen_harmonics: f32,
    pub storm: f32,
    pub distance: f32,
    pub decay: f32,
    pub freeze: f32,
    pub mix: f32,
    pub input: f32,
    pub output: f32,
}

impl From<FrostVerbFfiParams> for FrostVerbParams {
    fn from(value: FrostVerbFfiParams) -> Self {
        Self {
            coldness: value.coldness,
            wind: value.wind,
            ice_size: value.ice_size,
            ancient_depth: value.ancient_depth,
            frozen_harmonics: value.frozen_harmonics,
            storm: value.storm,
            distance: value.distance,
            decay: value.decay,
            freeze: value.freeze,
            mix: value.mix,
            input: value.input,
            output: value.output,
        }
        .sanitized()
    }
}

impl From<FrostVerbParams> for FrostVerbFfiParams {
    fn from(value: FrostVerbParams) -> Self {
        let value = value.sanitized();
        Self {
            coldness: value.coldness,
            wind: value.wind,
            ice_size: value.ice_size,
            ancient_depth: value.ancient_depth,
            frozen_harmonics: value.frozen_harmonics,
            storm: value.storm,
            distance: value.distance,
            decay: value.decay,
            freeze: value.freeze,
            mix: value.mix,
            input: value.input,
            output: value.output,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FrostVerbFfiMeters {
    pub input_peak: f32,
    pub output_peak: f32,
    pub wet_peak: f32,
    pub limiter_gain_reduction_db: f32,
}

impl From<EngineMeters> for FrostVerbFfiMeters {
    fn from(value: EngineMeters) -> Self {
        Self {
            input_peak: value.input_peak,
            output_peak: value.output_peak,
            wet_peak: value.wet_peak,
            limiter_gain_reduction_db: value.limiter_gain_reduction_db,
        }
    }
}

pub struct FrostVerbFfiEngine {
    engine: FrostVerbEngine,
}

#[unsafe(no_mangle)]
pub extern "C" fn frostverb_parameter_count() -> usize {
    PARAMETER_DEFS.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn frostverb_default_params() -> FrostVerbFfiParams {
    FrostVerbParams::default().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn frostverb_engine_create() -> *mut FrostVerbFfiEngine {
    Box::into_raw(Box::new(FrostVerbFfiEngine {
        engine: FrostVerbEngine::new(),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frostverb_engine_destroy(handle: *mut FrostVerbFfiEngine) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frostverb_engine_prepare(
    handle: *mut FrostVerbFfiEngine,
    sample_rate: f32,
) {
    if let Some(wrapper) = unsafe { handle.as_mut() } {
        wrapper.engine.prepare(sample_rate);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frostverb_engine_reset(handle: *mut FrostVerbFfiEngine) {
    if let Some(wrapper) = unsafe { handle.as_mut() } {
        wrapper.engine.reset();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frostverb_engine_process_interleaved(
    handle: *mut FrostVerbFfiEngine,
    buffer: *mut f32,
    frame_count: usize,
    params: FrostVerbFfiParams,
) {
    let Some(wrapper) = (unsafe { handle.as_mut() }) else {
        return;
    };
    if buffer.is_null() || frame_count == 0 {
        return;
    }

    let samples = unsafe { core::slice::from_raw_parts_mut(buffer, frame_count * 2) };
    wrapper
        .engine
        .process_interleaved_in_place(samples, &params.into());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frostverb_engine_meters(
    handle: *const FrostVerbFfiEngine,
) -> FrostVerbFfiMeters {
    unsafe { handle.as_ref() }
        .map(|wrapper| wrapper.engine.meters().into())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ffi_defaults_match_public_parameter_defaults() {
        assert_eq!(frostverb_parameter_count(), PARAMETER_DEFS.len());
        let defaults = frostverb_default_params();
        assert_eq!(defaults, FrostVerbParams::default().into());
    }

    #[test]
    fn ffi_engine_processes_interleaved_stereo() {
        let handle = frostverb_engine_create();
        assert!(!handle.is_null());

        unsafe {
            frostverb_engine_prepare(handle, 48_000.0);
        }

        let mut buffer = [0.5_f32, -0.25, 0.0, 0.0, 0.0, 0.0];
        unsafe {
            frostverb_engine_process_interleaved(
                handle,
                buffer.as_mut_ptr(),
                buffer.len() / 2,
                frostverb_default_params(),
            );
        }

        assert!(buffer.iter().all(|sample| sample.is_finite()));
        let meters = unsafe { frostverb_engine_meters(handle) };
        assert!(meters.input_peak > 0.0);

        unsafe {
            frostverb_engine_destroy(handle);
        }
    }
}
