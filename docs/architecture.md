# Frost Verb Architecture

## Crate Layout

- `src/params.rs`: stable public parameter contract for plugin wrappers and presets.
- `src/preset.rs`: factory preset data using the same parameter struct.
- `src/dsp/mod.rs`: high-level engine and stage routing.
- `src/dsp/early_reflections.rs`: asymmetric multi-tap early reflection layer.
- `src/dsp/fdn.rs`: 8-lane frozen diffusion network with Householder feedback, DC blocking, and feedback-path soft clipping.
- `src/dsp/resonance.rs`: ice resonance layer using modal bandpass filters.
- `src/dsp/wind.rs`: procedural stereo wind/noise layer.
- `src/dsp/freeze.rs`: realtime-safe spectral-hold inspired resonator freeze layer.
- `src/host.rs`: plugin metadata, preset lookup, normalized automation snapshot, and text state encoding for future plugin wrappers.
- `src/bin/frostverb-render.rs`: standalone offline renderer that writes a stereo demo WAV.

## Host Integration Plan

The DSP core is separated from the plugin wrapper so VST3, CLAP, standalone, UI, and tests can share one engine. The next host layer should:

- Map `PARAMETER_DEFS` into `nice-plug` parameters with the exact existing IDs.
- Smooth public parameters before or inside `FrostVerbEngine`.
- Allocate DSP buffers only during prepare/reset.
- Expose factory presets from `FACTORY_PRESETS`.
- Keep debug/developer parameters hidden from release builds.

The current repository does not vendor `nice-plug`, VST3 SDK, CLAP SDK, or egui. The stable host-facing contract is implemented locally first so an external wrapper can be added without changing parameter IDs, presets, or DSP state format.

## UI Plan

The UI should be an instrument panel, not a signal-flow dashboard. Layered PNG artwork supplies the visual identity; egui renders all text, values, controls, automation affordances, tooltips, preset menus, and view switching.

Primary layout:

- Large macro controls: Ice Size, Ancient Depth, Coldness.
- Secondary controls: Wind, Frozen Harmonics, Storm, Distance.
- Footer utilities: Input, Output, Mix, Freeze, preset selector.
- Optional views: Perform and Architecture.

Generated artwork must not include baked-in parameter labels, values, or host-automation state.
