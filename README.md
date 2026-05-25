# Frost Verb

Frost Verb is a Rust character-reverb core for a cold nordic sound-design plugin. The repository currently contains the product-grade DSP core, stable parameter and preset contracts, host-state scaffolding, tests, and an offline demo renderer.

## What Works Now

- Stereo DSP engine with early reflections, 8-lane FDN, ice resonance, wind, spectral-hold inspired freeze, DC blocking, smoothing, and safety limiting.
- Stable public parameter IDs in `src/params.rs`.
- Factory presets in code and `presets/factory_presets.json`.
- Host-facing descriptor, normalized parameter snapshot, preset lookup, and state encode/decode in `src/host.rs`.
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

The VST3/CLAP/egui wrapper is intentionally not faked in this repo because the required external plugin framework is not present. The DSP and host contracts are ready for that wrapper: keep the existing parameter IDs and use `FrostVerbEngine`, `HostParameterSnapshot`, `PARAMETER_DEFS`, and `FACTORY_PRESETS` as the single source of truth.
