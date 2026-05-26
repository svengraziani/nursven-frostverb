import { useId, type InputHTMLAttributes } from 'react';
import './FrostKnob.css';

type FrostKnobProps = {
  label: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  variant?: 'iceKnob' | 'runeKnob' | 'crystalKnob' | 'woodKnob';
  onChange?: (value: number) => void;
} & Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'value' | 'min' | 'max' | 'step' | 'onChange'>;

const variantRunes: Record<NonNullable<FrostKnobProps['variant']>, string> = {
  iceKnob: 'ᚠ ᚱ ᛟ ᛋ ᛏ ᚡ ᛖ ᚱ ᛒ ᚾ ᚢ ᚱ ᛋ ᚡ ᛖ ᚾ',
  runeKnob: 'ᚦ ᚨ ᚱ ᛁ ᛊ ᛟ ᚾ ᚨ ᚾ ᚦ ᚱ ᚢ ᛗ ᛖ',
  crystalKnob: 'ᛁ ᛊ ᚲ ᚱ ᛁ ᛊ ᛏ ᚨ ᛚ ᚠ ᚱ ᛖ ᛖ ᛉ ᛖ',
  woodKnob: 'ᚱ ᛖ ᚡ ᛖ ᚱ ᛒ ᛚ ᛖ ᚾ ᚷ ᛏ ᚺ ᚹ ᛟ ᛟ ᛞ',
};

const variantClass: Record<NonNullable<FrostKnobProps['variant']>, string> = {
  iceKnob: 'ice',
  runeKnob: 'rune',
  crystalKnob: 'crystal',
  woodKnob: 'wood',
};

export function FrostKnob({
  label,
  value,
  min = 0,
  max = 100,
  step = 1,
  unit = '%',
  variant = 'iceKnob',
  onChange,
  ...inputProps
}: FrostKnobProps) {
  const svgId = useId().replace(/:/g, '');
  const ratio = Math.min(1, Math.max(0, (value - min) / (max - min)));
  const angle = -136 + ratio * 272;
  const accentArc = `${Math.max(0.001, ratio * 430)} 999`;
  const ticks = Array.from({ length: 29 }, (_, index) => {
    const tickAngle = -136 + index * (272 / 28);
    const isMajor = index % 4 === 0;

    return (
      <line
        key={index}
        className={isMajor ? 'frost-knob-svg__tick frost-knob-svg__tick--major' : 'frost-knob-svg__tick'}
        x1="128"
        y1={isMajor ? '14' : '20'}
        x2="128"
        y2={isMajor ? '32' : '30'}
        transform={`rotate(${tickAngle} 128 128)`}
      />
    );
  });

  return (
    <label className={`frost-knob frost-knob--${variantClass[variant]}`}>
      <span className="frost-knob__label">{label}</span>
      <span className="frost-knob__dial">
        <svg
          className="frost-knob-svg"
          viewBox="0 0 256 256"
          role="img"
          aria-label={`${label} dial at ${value}${unit}`}
        >
          <defs>
            <radialGradient id={`${svgId}-ice-face`} cx="42%" cy="34%" r="72%">
              <stop offset="0" stopColor="#f5ffff" />
              <stop offset="0.2" stopColor="#9ec9d5" />
              <stop offset="0.48" stopColor="#355465" />
              <stop offset="0.78" stopColor="#13232c" />
              <stop offset="1" stopColor="#071017" />
            </radialGradient>
            <radialGradient id={`${svgId}-cap`} cx="36%" cy="32%" r="76%">
              <stop offset="0" stopColor="#f8ffff" />
              <stop offset="0.22" stopColor="#b7d6de" />
              <stop offset="0.43" stopColor="#5e808d" />
              <stop offset="0.66" stopColor="#263c47" />
              <stop offset="1" stopColor="#081116" />
            </radialGradient>
            <linearGradient id={`${svgId}-brass`} x1="42" y1="42" x2="214" y2="214">
              <stop offset="0" stopColor="#f3e4bd" />
              <stop offset="0.28" stopColor="#a98d59" />
              <stop offset="0.56" stopColor="#53402a" />
              <stop offset="0.82" stopColor="#bd9e64" />
              <stop offset="1" stopColor="#2f251b" />
            </linearGradient>
            <linearGradient id={`${svgId}-ice-edge`} x1="28" y1="12" x2="224" y2="238">
              <stop offset="0" stopColor="#e6fbff" />
              <stop offset="0.32" stopColor="#79bfd0" />
              <stop offset="0.7" stopColor="#213b48" />
              <stop offset="1" stopColor="#a8edff" />
            </linearGradient>
            <filter id={`${svgId}-glow`} x="-35%" y="-35%" width="170%" height="170%">
              <feGaussianBlur stdDeviation="3.2" result="blur" />
              <feColorMatrix
                in="blur"
                type="matrix"
                values="0 0 0 0 0.28 0 0 0 0 0.9 0 0 0 0 1 0 0 0 0.8 0"
              />
              <feMerge>
                <feMergeNode />
                <feMergeNode in="SourceGraphic" />
              </feMerge>
            </filter>
            <filter id={`${svgId}-roughen`}>
              <feTurbulence baseFrequency="0.95" numOctaves="3" seed="8" type="fractalNoise" />
              <feDisplacementMap in="SourceGraphic" scale="1.6" />
            </filter>
            <path id={`${svgId}-rune-path`} d="M 35 128 A 93 93 0 1 1 221 128 A 93 93 0 1 1 35 128" />
          </defs>

          <circle className="frost-knob-svg__shadow" cx="128" cy="137" r="110" />
          <circle className="frost-knob-svg__outer" cx="128" cy="128" r="111" fill={`url(#${svgId}-ice-edge)`} />
          <circle className="frost-knob-svg__outer-dark" cx="128" cy="128" r="104" />
          <circle className="frost-knob-svg__rune-band" cx="128" cy="128" r="94" fill={`url(#${svgId}-ice-face)`} />
          <circle
            className="frost-knob-svg__value-arc"
            cx="128"
            cy="128"
            r="76"
            pathLength="430"
            strokeDasharray={accentArc}
            transform="rotate(-136 128 128)"
            filter={`url(#${svgId}-glow)`}
          />
          {ticks}
          <text className="frost-knob-svg__runes">
            <textPath href={`#${svgId}-rune-path`} startOffset="9%">
              {variantRunes[variant]}
            </textPath>
          </text>
          <circle className="frost-knob-svg__brass" cx="128" cy="128" r="70" fill={`url(#${svgId}-brass)`} />
          <circle className="frost-knob-svg__brass-groove" cx="128" cy="128" r="59" />
          <circle className="frost-knob-svg__cap" cx="128" cy="128" r="54" fill={`url(#${svgId}-cap)`} />
          <g className="frost-knob-svg__brushing" opacity="0.45">
            <path d="M82 116c27-13 67-12 92 3" />
            <path d="M87 143c26 15 58 15 83 0" />
            <path d="M99 89c16 28 35 55 58 78" />
            <path d="M93 166c19-33 39-58 70-78" />
          </g>
          <g className="frost-knob-svg__pointer" transform={`rotate(${angle} 128 128)`}>
            <path d="M128 61 139 88 128 82 117 88Z" />
            <line x1="128" y1="54" x2="128" y2="83" />
          </g>
          <circle className="frost-knob-svg__ice-speckles" cx="128" cy="128" r="50" filter={`url(#${svgId}-roughen)`} />
          <circle className="frost-knob-svg__highlight" cx="102" cy="95" r="31" />
        </svg>
      </span>
      <span className="frost-knob__readout">
        {value}
        {unit}
      </span>
      <input
        {...inputProps}
        className="frost-knob__input"
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(event) => onChange?.(Number(event.currentTarget.value))}
      />
    </label>
  );
}
