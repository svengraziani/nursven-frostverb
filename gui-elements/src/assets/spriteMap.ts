import spriteSheetUrl from '../../res/vorlagen/elements-sprite-sheet.png';

export type SpriteId =
  | 'iceKnob'
  | 'runeKnob'
  | 'crystalKnob'
  | 'woodKnob'
  | 'verticalFaders'
  | 'horizontalFaders'
  | 'pluginPanel'
  | 'worldTreeEmblem';

export type SpriteFrame = {
  id: SpriteId;
  x: number;
  y: number;
  width: number;
  height: number;
};

export const SPRITE_SHEET = {
  url: spriteSheetUrl,
  width: 2752,
  height: 1536,
};

export const spriteFrames: Record<SpriteId, SpriteFrame> = {
  iceKnob: { id: 'iceKnob', x: 190, y: 360, width: 390, height: 385 },
  runeKnob: { id: 'runeKnob', x: 665, y: 365, width: 280, height: 325 },
  crystalKnob: { id: 'crystalKnob', x: 205, y: 1015, width: 330, height: 345 },
  woodKnob: { id: 'woodKnob', x: 1805, y: 965, width: 360, height: 390 },
  verticalFaders: { id: 'verticalFaders', x: 2215, y: 205, width: 390, height: 690 },
  horizontalFaders: { id: 'horizontalFaders', x: 1045, y: 1000, width: 610, height: 345 },
  pluginPanel: { id: 'pluginPanel', x: 975, y: 145, width: 1110, height: 735 },
  worldTreeEmblem: { id: 'worldTreeEmblem', x: 2290, y: 1005, width: 330, height: 350 },
};
