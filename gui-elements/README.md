# Frostverb GUI Elements

React component workspace for the Frostverb audio-plugin WebUI. The app uses Vite for fast local development and Storybook for isolated component design.

## Commands

- `npm run dev` starts the Vite app.
- `npm run storybook` starts the component workshop.
- `npm run build` type-checks and builds the Vite app.
- `npm run build-storybook` builds static Storybook output.
- `npm run lint` checks TypeScript and React lint rules.

## Assets

- Text display font: `res/fonts/norse`
- Rune/code font: `res/fonts/norse_code`
- First visual source sheet: `res/vorlagen/elements-sprite-sheet.png`

The sprite sheet is mapped in `src/assets/spriteMap.ts`. Reusable crops are rendered by `SpriteCrop`, then composed into controls such as `FrostKnob`, `FrostSlider`, and `FrostPluginPanel`.
