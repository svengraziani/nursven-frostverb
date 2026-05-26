import type { CSSProperties } from 'react';
import { SPRITE_SHEET, spriteFrames, type SpriteId } from '../assets/spriteMap';
import './SpriteCrop.css';

type SpriteCropProps = {
  id: SpriteId;
  scale?: number;
  className?: string;
  label?: string;
};

export function SpriteCrop({ id, scale = 1, className = '', label }: SpriteCropProps) {
  const frame = spriteFrames[id];

  const style = {
    '--sprite-width': `${frame.width * scale}px`,
    '--sprite-height': `${frame.height * scale}px`,
    '--sheet-width': `${SPRITE_SHEET.width * scale}px`,
    '--sheet-height': `${SPRITE_SHEET.height * scale}px`,
    '--sprite-x': `${-frame.x * scale}px`,
    '--sprite-y': `${-frame.y * scale}px`,
  } as CSSProperties;

  return (
    <span
      aria-label={label}
      aria-hidden={label ? undefined : true}
      className={`sprite-crop ${className}`}
      style={style}
    >
      <img src={SPRITE_SHEET.url} alt="" draggable={false} />
    </span>
  );
}
