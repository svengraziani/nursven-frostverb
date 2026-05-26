import { useId } from 'react';
import './FrostMeter.css';

type FrostMeterChannel = {
  label?: string;
  value: number;
};

type FrostMeterProps = {
  label?: string;
  value?: number;
  channels?: FrostMeterChannel[];
  min?: number;
  max?: number;
  width?: number;
};

function clampRatio(value: number, min: number, max: number) {
  if (max <= min) return 0;

  return Math.min(1, Math.max(0, (value - min) / (max - min)));
}

export function FrostMeter({ label, value = 0, channels, min = 0, max = 100, width = 520 }: FrostMeterProps) {
  const svgId = useId().replace(/:/g, '');
  const meterChannels = channels?.length ? channels : [{ value }];
  const hasChannelLabels = meterChannels.some((channel) => channel.label);
  const height = meterChannels.length > 1 ? 134 : 82;
  const frameX = 4;
  const frameY = 7;
  const frameWidth = width - 8;
  const frameHeight = height - 14;
  const labelGutter = hasChannelLabels ? 48 : 20;
  const trackX = frameX + labelGutter;
  const trackWidth = frameWidth - labelGutter - 24;
  const trackHeight = 28;
  const firstTrackY = meterChannels.length > 1 ? 28 : 27;
  const trackGap = 42;
  const framePath = [
    `M ${frameX + 22} ${frameY}`,
    `L ${frameX + frameWidth - 22} ${frameY}`,
    `L ${frameX + frameWidth} ${frameY + 22}`,
    `L ${frameX + frameWidth} ${frameY + frameHeight - 22}`,
    `L ${frameX + frameWidth - 22} ${frameY + frameHeight}`,
    `L ${frameX + 22} ${frameY + frameHeight}`,
    `L ${frameX} ${frameY + frameHeight - 22}`,
    `L ${frameX} ${frameY + 22}`,
    'Z',
  ].join(' ');

  return (
    <figure className="frost-meter" style={{ width }} aria-label={label}>
      {label ? <figcaption className="frost-meter__label">{label}</figcaption> : null}
      <svg className="frost-meter__svg" viewBox={`0 0 ${width} ${height}`} role="img" aria-label={label ?? 'Frost meter'}>
        <defs>
          <linearGradient id={`${svgId}-frame`} x1="0" y1="0" x2="1" y2="1">
            <stop offset="0" stopColor="#9fc7d5" />
            <stop offset="0.16" stopColor="#3a5a65" />
            <stop offset="0.46" stopColor="#172832" />
            <stop offset="0.7" stopColor="#577987" />
            <stop offset="1" stopColor="#091116" />
          </linearGradient>
          <linearGradient id={`${svgId}-track`} x1="0" y1="0" x2="0" y2="1">
            <stop offset="0" stopColor="#0a2029" />
            <stop offset="0.55" stopColor="#071016" />
            <stop offset="1" stopColor="#03070a" />
          </linearGradient>
          <linearGradient id={`${svgId}-fill`} x1="0" y1="0" x2="1" y2="0">
            <stop offset="0" stopColor="#5beeff" />
            <stop offset="0.55" stopColor="#8bf7ff" />
            <stop offset="1" stopColor="#d8ffff" />
          </linearGradient>
          <pattern id={`${svgId}-ice-grain`} width="72" height="42" patternUnits="userSpaceOnUse">
            <rect width="72" height="42" fill="#7fa5b2" opacity="0.16" />
            <path d="M3 9h28M39 6h26M11 31h41M55 26h12" stroke="#e8fbff" strokeWidth="1.2" opacity="0.34" />
            <path d="M8 18h18M34 21h30M2 39h21" stroke="#04090c" strokeWidth="1.8" opacity="0.38" />
          </pattern>
          <pattern id={`${svgId}-fill-grain`} width="34" height="18" patternUnits="userSpaceOnUse">
            <path d="M2 5h9M15 4h16M4 13h17M25 12h7" stroke="#e9ffff" strokeWidth="1.2" opacity="0.46" />
            <path d="M1 9h5M12 9h8M26 7h6" stroke="#1fcde5" strokeWidth="2" opacity="0.32" />
          </pattern>
          <filter id={`${svgId}-rough-frame`} x="-8%" y="-22%" width="116%" height="144%">
            <feTurbulence baseFrequency="0.055" numOctaves="4" seed="17" type="fractalNoise" result="noise" />
            <feDisplacementMap in="SourceGraphic" in2="noise" scale="3.2" xChannelSelector="R" yChannelSelector="G" />
          </filter>
          <filter id={`${svgId}-frame-shadow`} x="-10%" y="-25%" width="120%" height="150%">
            <feDropShadow dx="0" dy="5" stdDeviation="4" floodColor="#000" floodOpacity="0.72" />
            <feDropShadow dx="0" dy="-1" stdDeviation="1.8" floodColor="#d8fbff" floodOpacity="0.22" />
          </filter>
          <filter id={`${svgId}-fill-glow`} x="-10%" y="-80%" width="120%" height="260%">
            <feDropShadow dx="0" dy="0" stdDeviation="5" floodColor="#5eeaff" floodOpacity="0.82" />
            <feDropShadow dx="0" dy="0" stdDeviation="12" floodColor="#5eeaff" floodOpacity="0.28" />
          </filter>
          <filter id={`${svgId}-label-glow`} x="-80%" y="-80%" width="260%" height="260%">
            <feDropShadow dx="0" dy="0" stdDeviation="3" floodColor="#95efff" floodOpacity="0.72" />
            <feDropShadow dx="2" dy="3" stdDeviation="2" floodColor="#000" floodOpacity="0.72" />
          </filter>
          {meterChannels.map((_, index) => (
            <clipPath key={index} id={`${svgId}-track-clip-${index}`}>
              <rect x={trackX + 6} y={firstTrackY + index * trackGap + 5} width={trackWidth - 12} height={trackHeight - 10} rx="8" />
            </clipPath>
          ))}
        </defs>

        <path className="frost-meter__shadow" d={framePath} />
        <path className="frost-meter__frame" d={framePath} fill={`url(#${svgId}-frame)`} filter={`url(#${svgId}-frame-shadow)`} />
        <path className="frost-meter__grain" d={framePath} fill={`url(#${svgId}-ice-grain)`} filter={`url(#${svgId}-rough-frame)`} />
        <path className="frost-meter__rim" d={framePath} />

        {meterChannels.map((channel, index) => {
          const y = firstTrackY + index * trackGap;
          const ratio = clampRatio(channel.value, min, max);
          const fillWidth = Math.max(0, (trackWidth - 12) * ratio);

          return (
            <g key={`${channel.label ?? 'meter'}-${index}`}>
              {channel.label ? (
                <text className="frost-meter__channel-label" x={frameX + 27} y={y + 20} filter={`url(#${svgId}-label-glow)`}>
                  {channel.label}
                </text>
              ) : null}
              <rect className="frost-meter__track-shadow" x={trackX - 2} y={y - 1} width={trackWidth + 4} height={trackHeight + 2} rx="12" />
              <rect className="frost-meter__track" x={trackX} y={y} width={trackWidth} height={trackHeight} rx="11" fill={`url(#${svgId}-track)`} />
              <g clipPath={`url(#${svgId}-track-clip-${index})`}>
                <rect className="frost-meter__fill" x={trackX + 6} y={y + 5} width={fillWidth} height={trackHeight - 10} fill={`url(#${svgId}-fill)`} filter={`url(#${svgId}-fill-glow)`} />
                <rect className="frost-meter__fill-texture" x={trackX + 6} y={y + 5} width={fillWidth} height={trackHeight - 10} fill={`url(#${svgId}-fill-grain)`} />
                <path className="frost-meter__fill-scan" d={`M ${trackX + 12} ${y + 10} H ${trackX + fillWidth}`} />
              </g>
              <rect className="frost-meter__track-rim" x={trackX} y={y} width={trackWidth} height={trackHeight} rx="11" />
            </g>
          );
        })}
      </svg>
    </figure>
  );
}
