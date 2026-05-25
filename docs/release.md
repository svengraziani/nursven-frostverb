# Frost Verb Release Checklist

## Local Verification

Run:

```sh
cargo check
cargo test
cargo run --bin frostverb-render -- target/frostverb-demo.wav "Frozen Cave"
```

After the `nice-plug` wrapper is added, also run:

```sh
cargo xtask bundle frostverb --release
```

## Plugin Smoke Tests

- Load VST3 in Ableton.
- Load CLAP in a CLAP-capable host.
- Launch standalone app.
- Test mono input, stereo input, silence, impulse, sine sweep, and pink noise.
- Automate every public parameter across the full range.
- Soak-test extreme settings for at least 10 minutes.
- Profile 44.1 kHz, 48 kHz, and 96 kHz sessions.

## macOS Distribution

Before public distribution, document and automate:

- Bundle output paths.
- VST3 and CLAP install paths.
- Codesign identity selection.
- Gatekeeper/notarization flow.
- Versioning and preset compatibility policy.
