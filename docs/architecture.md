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
- `src/ffi.rs`: C ABI for the JUCE/C++ plugin shell.
- `src/ui.rs`: optional `gui` feature with the native egui panel, PNG layer manifest, preset selector, meters, and host-facing parameter changes.
- `src/bin/frostverb-render.rs`: standalone offline renderer that writes a stereo demo WAV.
- `plugin/`: JUCE 8 C++ plugin shell with VST3/AU/standalone targets and a WebView editor.

## Host Integration Plan

The DSP core is separated from the plugin wrapper so VST3, AU, standalone, UI, and tests can share one engine. The JUCE host layer should:

- Mirror `PARAMETER_DEFS` in `plugin/Source/ParameterIds.h` with the exact existing IDs and order.
- Call the Rust DSP through `plugin/include/frostverb/frostverb_ffi.h`.
- Smooth public parameters before or inside `FrostVerbEngine`.
- Allocate DSP buffers only during prepare/reset.
- Expose factory presets from `FACTORY_PRESETS`.
- Keep debug/developer parameters hidden from release builds.

The current repository does not vendor JUCE. Configure `plugin/` with `FROSTVERB_JUCE_DIR` or `FROSTVERB_FETCH_JUCE=ON`. The stable host-facing contract is implemented locally first so the wrapper can evolve without changing parameter IDs, presets, DSP state format, or WebView control IDs.

## UI Plan

The production UI direction is the JUCE 8 WebView editor in `plugin/web`. It should feel like an instrument panel, not a signal-flow dashboard. HTML/CSS/JavaScript, SVG/canvas/WebGL, and future PNG artwork supply the visual identity; JUCE owns host automation, state, and plugin lifecycle.

Primary layout:

- Large macro controls: Ice Size, Ancient Depth, Coldness.
- Secondary controls: Wind, Frozen Harmonics, Storm, Distance.
- Footer utilities: Input, Output, Mix, Freeze, preset selector.
- Optional views: Perform and Architecture.

Generated artwork must not include baked-in parameter labels, values, or host-automation state.

The fixed design surface is `1000x620` and scales proportionally inside the WebView. The older optional Rust/egui module remains as a native scaffold, but the active plugin path is JUCE/WebView.
