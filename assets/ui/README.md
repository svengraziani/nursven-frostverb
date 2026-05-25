# Frost Verb UI Artwork

PNG files in this directory are intentionally split by purpose:

- `base_panel.png`: full-size 1000x620 background panel.
- `ice_structure_overlay.png` and `foreground_scratches.png`: static texture overlays without text.
- `large_knob_cap.png`, `medium_knob_cap.png`, `small_knob_cap.png`, `toggle_plate.png`: reusable control artwork.
- `freeze_glow.png`, `storm_wind.png`, `coldness_ice.png`, `meter_accent.png`: reactive overlays mixed by parameter or meter values.

Do not bake parameter names, values, preset names, or automation state into these PNGs. The egui layer renders all text and remains responsible for interaction.
