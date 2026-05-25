# Frost Verb JUCE Plugin

This is the JUCE 8 plugin shell for Frost Verb. The C++ layer owns plugin formats, host parameters, state, and the WebView editor. The Rust crate remains the DSP source of truth through `plugin/include/frostverb/frostverb_ffi.h`.

Configure with a local JUCE checkout:

```sh
cmake -S plugin -B build/plugin -DFROSTVERB_JUCE_DIR=/path/to/JUCE
cmake --build build/plugin
```

Or allow CMake to fetch JUCE:

```sh
cmake -S plugin -B build/plugin -DFROSTVERB_FETCH_JUCE=ON
cmake --build build/plugin
```

The C++ parameter IDs in `Source/ParameterIds.h` intentionally mirror `src/params.rs` exactly.
