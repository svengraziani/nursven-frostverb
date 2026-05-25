# Frost Verb

Frost Verb is a Rust character-reverb core with a JUCE 8 plugin shell for a cold nordic sound-design plugin. The repository currently contains the DSP core, stable parameter and preset contracts, host-state scaffolding, tests, an offline demo renderer, and a C++ JUCE/WebView plugin prototype.

## What Works Now

- Stereo DSP engine with early reflections, 8-lane FDN, ice resonance, wind, spectral-hold inspired freeze, DC blocking, smoothing, and safety limiting.
- Stable public parameter IDs in `src/params.rs`.
- Factory presets in code and `presets/factory_presets.json`.
- Host-facing descriptor, normalized parameter snapshot, preset lookup, and state encode/decode in `src/host.rs`.
- C ABI in `src/ffi.rs` for native wrappers.
- JUCE 8 C++ plugin shell in `plugin/` with mirrored parameter IDs, Rust DSP calls, and a WebView proof-of-concept editor.
- Offline standalone renderer:

```sh
cargo run --bin frostverb-render -- target/frostverb-demo.wav "Frozen Cave"
```

- Dummy host backend that simulates a plugin host with block processing, preset load, state automation, meters, and WAV output:

```sh
cargo run --bin frostverb-dummy-host -- target/frostverb-dummy-host.wav "Whiteout"
```

## Verification

```sh
cargo fmt --check
cargo check
cargo test
```

## Plugin Wrapper Status

The active plugin direction is JUCE 8 with a WebView editor. Configure it with a local JUCE checkout or allow CMake to fetch JUCE:

```sh
cmake -S plugin -B build/plugin -DFROSTVERB_JUCE_DIR=/path/to/JUCE
cmake --build build/plugin --target FrostVerb_VST3
```

To build and run the standalone demo app with CMake fetching JUCE:

```sh
cmake -S plugin -B build/plugin -DFROSTVERB_FETCH_JUCE=ON
cmake --build build/plugin --target FrostVerb_Standalone --config Debug -j 4
open "build/plugin/FrostVerb_artefacts/Standalone/Frost Verb.app"
```

If you are using the temporary build from the first scaffold verification, run:

```sh
open "/private/tmp/frostverb-plugin-fetch/FrostVerb_artefacts/Standalone/Frost Verb.app"
```

The Rust DSP and C++ wrapper share the same public parameter IDs. `tests/product_contract.rs` checks that `plugin/Source/ParameterIds.h` stays in the same order as `src/params.rs`.
