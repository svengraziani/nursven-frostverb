import { useId, useRef, type InputHTMLAttributes, type PointerEvent, type WheelEvent } from 'react';
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

const dirtMap = new URL('../../knob/dirtmap.png', import.meta.url).href;
const dirtMap2 = new URL('../../knob/dirtmap-2.png', import.meta.url).href;

const dialMinAngle = -135;
const dialMaxAngle = 135;
const dragPixelsForFullRange = 180;

const variantRunes: Record<NonNullable<FrostKnobProps['variant']>, string> = {
  iceKnob: 'FROSTVERBNURSVEND',
  runeKnob: 'THARISONGRUMES',
  crystalKnob: 'ISKRISTALFREEZE',
  woodKnob: 'REVERBLENGTHWOOD',
};

const variantClass: Record<NonNullable<FrostKnobProps['variant']>, string> = {
  iceKnob: 'ice',
  runeKnob: 'rune',
  crystalKnob: 'crystal',
  woodKnob: 'wood',
};

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value));
}

function getStepPrecision(step: number) {
  const [, decimals = ''] = String(step).split('.');

  return decimals.length;
}

function quantizeValue(value: number, min: number, max: number, step: number) {
  if (step <= 0) {
    return clamp(value, min, max);
  }

  const precision = getStepPrecision(step);
  const stepped = Math.round((value - min) / step) * step + min;

  return Number(clamp(stepped, min, max).toFixed(precision));
}

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
  const dragStartYRef = useRef(0);
  const dragStartValueRef = useRef(value);
  const range = max - min || 1;
  const ratio = clamp((value - min) / range, 0, 1);
  const angle = dialMinAngle + ratio * (dialMaxAngle - dialMinAngle);
  const runes = variantRunes[variant].split('');

  const setDelta = (delta: number) => {
    onChange?.(quantizeValue(value + delta, min, max, step));
  };

  const handlePointerDown = (event: PointerEvent<SVGSVGElement>) => {
    event.preventDefault();
    dragStartYRef.current = event.clientY;
    dragStartValueRef.current = value;
    event.currentTarget.setPointerCapture(event.pointerId);
  };

  const handlePointerMove = (event: PointerEvent<SVGSVGElement>) => {
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      const dragDistance = dragStartYRef.current - event.clientY;
      const dragValue = (dragDistance / dragPixelsForFullRange) * range;

      onChange?.(quantizeValue(dragStartValueRef.current + dragValue, min, max, step));
    }
  };

  const handlePointerEnd = (event: PointerEvent<SVGSVGElement>) => {
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  };

  const handleWheel = (event: WheelEvent<SVGSVGElement>) => {
    event.preventDefault();
    setDelta(event.deltaY < 0 ? step : -step);
  };

  return (
    <label className={`frost-knob frost-knob--${variantClass[variant]}`}>
      <span className="frost-knob__label">{label}</span>
      <span className="frost-knob__dial">
        <svg
          className="frost-knob-svg"
          viewBox="0 0 420 420"
          role="img"
          aria-label={`${label} dial at ${value}${unit}`}
          style={{ '--dial-ratio': ratio, '--dial-angle': `${angle}deg` } as React.CSSProperties}
          onPointerDown={handlePointerDown}
          onPointerMove={handlePointerMove}
          onPointerUp={handlePointerEnd}
          onPointerCancel={handlePointerEnd}
          onWheel={handleWheel}
        >
          <defs>
            <clipPath id={`${svgId}-outer-clip`}>
              <circle cx="210" cy="190" r="151" />
            </clipPath>
            <clipPath id={`${svgId}-rune-ring-clip`}>
              <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M210 55a135 135 0 1 1 0 270a135 135 0 1 1 0-270M210 98a92 92 0 1 0 0 184a92 92 0 1 0 0-184"
              />
            </clipPath>
            <clipPath id={`${svgId}-cap-clip`}>
              <circle cx="210" cy="190" r="79" />
            </clipPath>
            <radialGradient id={`${svgId}-outer-body-gradient`} cx="38%" cy="23%" r="76%">
              <stop offset="0" stopColor="#8fb9cc" />
              <stop offset=".25" stopColor="#33586d" />
              <stop offset=".58" stopColor="#0d2535" />
              <stop offset="1" stopColor="#030b14" />
            </radialGradient>
            <radialGradient id={`${svgId}-rune-ring-gradient`} cx="38%" cy="22%" r="82%">
              <stop offset="0" stopColor="#8ebed1" />
              <stop offset=".22" stopColor="#426a80" />
              <stop offset=".52" stopColor="#173447" />
              <stop offset=".78" stopColor="#071722" />
              <stop offset="1" stopColor="#02070d" />
            </radialGradient>
            <radialGradient id={`${svgId}-cap-gradient`} cx="39%" cy="28%" r="76%">
              <stop offset="0" stopColor="#bfefff" />
              <stop offset=".18" stopColor="#6f9fb6" />
              <stop offset=".46" stopColor="#294f65" />
              <stop offset=".72" stopColor="#0b2333" />
              <stop offset="1" stopColor="#030a12" />
            </radialGradient>
            <linearGradient id={`${svgId}-bevel-gradient`} x1="120" y1="90" x2="300" y2="295">
              <stop offset="0" stopColor="#aeefff" />
              <stop offset=".24" stopColor="#3f748d" />
              <stop offset=".54" stopColor="#06111c" />
              <stop offset=".78" stopColor="#1b4963" />
              <stop offset="1" stopColor="#030a12" />
            </linearGradient>
            <pattern id={`${svgId}-dirt-pattern`} width="92" height="92" patternUnits="userSpaceOnUse">
              <image href={dirtMap2} width="92" height="92" preserveAspectRatio="none" />
            </pattern>
            <pattern id={`${svgId}-cap-dirt-pattern`} width="76" height="76" patternUnits="userSpaceOnUse">
              <image href={dirtMap} width="76" height="76" preserveAspectRatio="none" />
            </pattern>
            <filter id={`${svgId}-outer-shadow`} x="-30%" y="-30%" width="160%" height="170%">
              <feDropShadow dx="0" dy="16" stdDeviation="11" floodColor="#000" floodOpacity=".72" />
              <feDropShadow dx="0" dy="-3" stdDeviation="3" floodColor="#000" floodOpacity=".24" />
            </filter>
            <filter id={`${svgId}-frayed-edge`} x="-18%" y="-18%" width="136%" height="136%">
              <feTurbulence baseFrequency="0.09" numOctaves="10" seed="22" type="fractalNoise" result="noise" />
              <feDisplacementMap
                in="SourceGraphic"
                in2="noise"
                scale="40"
                xChannelSelector="R"
                yChannelSelector="G"
              />
            </filter>
            <filter id={`${svgId}-soft-blur`}>
              <feGaussianBlur stdDeviation="1" />
            </filter>
            <filter id={`${svgId}-cap-line-blur`}>
              <feGaussianBlur stdDeviation="1.4" />
            </filter>
            <filter id={`${svgId}-inner-well-organic`} x="-25%" y="-25%" width="150%" height="150%">
              <feTurbulence baseFrequency=".045" numOctaves="3" seed="41" type="fractalNoise" result="ring-noise" />
              <feDisplacementMap
                in="SourceGraphic"
                in2="ring-noise"
                scale="2.2"
                xChannelSelector="R"
                yChannelSelector="G"
                result="warped-ring"
              />
              <feDropShadow in="warped-ring" dx="0" dy="6" stdDeviation="5" floodColor="#000" floodOpacity=".78" />
            </filter>
            <filter id={`${svgId}-bevel-organic`} x="-35%" y="-35%" width="170%" height="170%">
              <feTurbulence baseFrequency=".052" numOctaves="3" seed="67" type="fractalNoise" result="bevel-noise" />
              <feDisplacementMap
                in="SourceGraphic"
                in2="bevel-noise"
                scale="2.8"
                xChannelSelector="R"
                yChannelSelector="G"
                result="warped-bevel"
              />
              <feDropShadow in="warped-bevel" dx="0" dy="3" stdDeviation="2.5" floodColor="#000" floodOpacity=".58" />
              <feDropShadow dx="0" dy="0" stdDeviation="3.4" floodColor="#8df5ff" floodOpacity=".18" />
            </filter>
            <filter id={`${svgId}-rim-organic`} x="-18%" y="-18%" width="136%" height="136%">
              <feTurbulence baseFrequency=".075" numOctaves="2" seed="89" type="fractalNoise" result="rim-noise" />
              <feDisplacementMap
                in="SourceGraphic"
                in2="rim-noise"
                scale="3.5"
                xChannelSelector="R"
                yChannelSelector="G"
              />
            </filter>
            <filter id={`${svgId}-small-shadow`} x="-35%" y="-35%" width="170%" height="170%">
              <feDropShadow dx="0" dy="3" stdDeviation="2.5" floodColor="#000" floodOpacity=".58" />
            </filter>
            <filter id={`${svgId}-cap-shadow`} x="-35%" y="-35%" width="170%" height="170%">
              <feDropShadow dx="0" dy="8" stdDeviation="6" floodColor="#000" floodOpacity=".68" />
              <feDropShadow dx="0" dy="-2" stdDeviation="2" floodColor="#e9fbff" floodOpacity=".25" />
            </filter>
            <filter id={`${svgId}-rune-glow`} x="-20%" y="-20%" width="140%" height="140%">
              <feDropShadow dx="0" dy="0" stdDeviation="5" floodColor="#a7efff" floodOpacity="1" />
              <feDropShadow dx="0" dy="0" stdDeviation="5" floodColor="#a7efff" floodOpacity="1" />
            </filter>
          </defs>

          <circle className="frost-knob-svg__outer-body" cx="210" cy="190" r="151" fill={`url(#${svgId}-outer-body-gradient)`} filter={`url(#${svgId}-outer-shadow)`} />
          <circle className="frost-knob-svg__outer-frost" cx="210" cy="190" r="151" filter={`url(#${svgId}-frayed-edge)`} />
          <g clipPath={`url(#${svgId}-outer-clip)`}>
            <image
              className="frost-knob-svg__ice-dust"
              href={dirtMap2}
              x="44"
              y="24"
              width="332"
              height="332"
              preserveAspectRatio="xMidYMid slice"
            />
            <rect className="frost-knob-svg__ice-dust" x="44" y="24" width="332" height="332" fill={`url(#${svgId}-dirt-pattern)`} />
          </g>
          <circle className="frost-knob-svg__rune-ring" cx="210" cy="190" r="128" fill={`url(#${svgId}-rune-ring-gradient)`} />
          <g clipPath={`url(#${svgId}-rune-ring-clip)`}>
            <rect x="72" y="52" width="276" height="276" fill={`url(#${svgId}-dirt-pattern)`} opacity=".22" />
            <circle
              className="frost-knob-svg__rune-ring-inner-shadow"
              cx="210"
              cy="190"
              r="98"
              filter={`url(#${svgId}-soft-blur)`}
            />
          </g>
          <g aria-hidden="true">
            {runes.map((rune, index) => {
              const runeRatio = runes.length === 1 ? 0 : index / (runes.length - 1);
              const runeAngle = dialMinAngle + runeRatio * (dialMaxAngle - dialMinAngle);
              const radians = (runeAngle * Math.PI) / 180;
              const x = 210 + Math.sin(radians) * 121;
              const y = 190 - Math.cos(radians) * 121;
              const isLit = angle >= runeAngle;

              return (
                <text
                  key={`${rune}-${index}`}
                  className={isLit ? 'frost-knob-svg__rune-glyph is-lit' : 'frost-knob-svg__rune-glyph'}
                  x={x.toFixed(2)}
                  y={y.toFixed(2)}
                  transform={`rotate(${runeAngle.toFixed(2)} ${x.toFixed(2)} ${y.toFixed(2)})`}
                  filter={isLit ? `url(#${svgId}-rune-glow)` : undefined}
                >
                  {rune}
                </text>
              );
            })}
          </g>
          <circle className="frost-knob-svg__inner-well" cx="210" cy="190" r="93" filter={`url(#${svgId}-inner-well-organic)`} />
          <circle className="frost-knob-svg__bevel-ring" cx="210" cy="190" r="87" stroke={`url(#${svgId}-bevel-gradient)`} filter={`url(#${svgId}-bevel-organic)`} />
          <g>
            <circle className="frost-knob-svg__cap-face" cx="210" cy="190" r="79" fill={`url(#${svgId}-cap-gradient)`} filter={`url(#${svgId}-cap-shadow)`} />
            <g clipPath={`url(#${svgId}-cap-clip)`}>
              <rect className="frost-knob-svg__cap-texture" x="131" y="111" width="158" height="158" fill={`url(#${svgId}-cap-dirt-pattern)`} />
              <g className="frost-knob-svg__cap-radial-lines">
                <path d="M210 190L154 119a96 96 0 0 1 56-18z" filter={`url(#${svgId}-cap-line-blur)`} />
                <path d="M210 190l78-22a96 96 0 0 1-15 74z" filter={`url(#${svgId}-cap-line-blur)`} />
                <path d="M210 190l-69 42a96 96 0 0 1-10-70z" filter={`url(#${svgId}-cap-line-blur)`} />
                <path d="M210 190l11 80a96 96 0 0 1-61-21z" filter={`url(#${svgId}-cap-line-blur)`} />
              </g>
            </g>
            <circle className="frost-knob-svg__cap-rim-dark" cx="210" cy="190" r="79" filter={`url(#${svgId}-rim-organic)`} />
            <circle className="frost-knob-svg__cap-rim-light" cx="210" cy="190" r="75" filter={`url(#${svgId}-rim-organic)`} />
          </g>
          <g className="frost-knob-svg__cap-indicator">
            <path className="frost-knob-svg__top-pointer" d="M210 89l12 35h-24z" filter={`url(#${svgId}-small-shadow)`} />
            <path className="frost-knob-svg__cap-triangle" d="M210 135l8 15h-16z" />
          </g>
          <text className="frost-knob-svg__limit-label" x="92" y="356">
            {min}
          </text>
          <text className="frost-knob-svg__limit-label" x="328" y="356">
            {max}
          </text>
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
