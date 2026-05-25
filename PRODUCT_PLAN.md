# Frost Verb Product Plan

Frost Verb is planned as a full character reverb product, not a reduced MVP. The sound target is a cold, nordic, sound-design-focused reverb with stable plugin architecture, factory presets, host automation, parameter smoothing, gain safety, and room for future DSP expansion.

## Product Surface

- Stereo audio effect plugin.
- VST3 and CLAP builds through the planned `nice-plug` host wrapper.
- Standalone build for development and demos.
- egui UI with layered PNG artwork and native controls.
- Factory preset system with stable parameter IDs.
- Host automation for every public sound parameter.
- No AI, ML, or GPU dependency in version 1.

## Public Parameters

The public parameter contract is defined in `src/params.rs`. IDs must not be renamed after release:

- `coldness`
- `wind`
- `ice_size`
- `ancient_depth`
- `frozen_harmonics`
- `storm`
- `distance`
- `decay`
- `freeze`
- `mix`
- `input`
- `output`

## DSP Signal Path

`Input -> Input Conditioning -> Early Reflections -> Frozen Diffusion Network -> Ice Resonance Layer -> Wind Layer -> Spectral Freeze -> Output Conditioning`

The current implementation provides the initial realtime-safe DSP core for each stage. `SpectralFreezeStub` is intentionally a bounded placeholder so routing, automation, smoothing, and safety can be tested before adding FFT hold logic.

## Release Criteria

- No audio-thread allocations in steady-state processing.
- No denormals or CPU spikes on silence.
- No feedback explosion at extreme settings.
- Parameter automation without clicks.
- Mono and stereo sources process cleanly.
- Presets remain compatible across versions.
- VST3, CLAP, standalone, install, and codesign steps are documented before first public release.
