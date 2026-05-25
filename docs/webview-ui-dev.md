# WebView UI Development

Frost Verb's production UI lives in `plugin/web/` and is embedded by the JUCE 8 editor. This is the fastest place to iterate on the visual design.

## Files

- `plugin/web/index.html`: UI structure.
- `plugin/web/styles.css`: visual design, layout, typography, animation.
- `plugin/web/app.js`: parameter interaction, JUCE bridge calls, WebGL background.

The JUCE side embeds these files through `juce_add_binary_data` in `plugin/CMakeLists.txt`.

## Browser Iteration

Run a local static server from the repository root:

```sh
python3 -m http.server 5173 --directory plugin/web
```

Open:

```text
http://localhost:5173
```

Use this browser loop for art direction, layout, CSS, SVG, canvas, and WebGL work. The browser preview will not have the JUCE native bridge, so `app.js` must keep fallback defaults for parameter values.

## JUCE Standalone Test

After changing files in `plugin/web`, rebuild the JUCE target so the assets are re-embedded:

```sh
cmake --build build/plugin --target FrostVerb_Standalone --config Debug -j 4
open "build/plugin/FrostVerb_artefacts/Standalone/Frost Verb.app"
```

If using the temporary verification build:

```sh
cmake --build /private/tmp/frostverb-plugin-fetch --target FrostVerb_Standalone --config Debug -j 4
open "/private/tmp/frostverb-plugin-fetch/FrostVerb_artefacts/Standalone/Frost Verb.app"
```

## Design Workflow

Start with a strong static art direction before adding complex motion:

1. Nail the panel composition, spacing, color system, typography, and control hierarchy.
2. Make all parameters readable and reachable.
3. Add hover/drag feedback.
4. Add SVG/canvas/WebGL polish.
5. Test inside the JUCE standalone app.

Avoid baking labels, values, preset names, or automation state into image assets. These must remain dynamic so host automation and state updates can be reflected correctly.

## GPT Editing Prompt

When asking GPT/Codex to improve the UI, point it at `plugin/web/` directly. Example:

```text
Make the UI in plugin/web feel like a premium icy nordic reverb plugin.
Keep the existing JUCE bridge calls, keep all parameter IDs unchanged, and improve
the HTML/CSS/JS visual design with responsive layout and polished controls.
```

Important constraints:

- Keep parameter IDs exactly aligned with `src/params.rs`.
- Do not remove the `setParameter` native bridge call in `app.js`.
- Keep browser fallbacks so the UI can run outside JUCE during design.
- Rebuild the JUCE target after editing web assets.
