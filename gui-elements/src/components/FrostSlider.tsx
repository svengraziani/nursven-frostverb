import type { CSSProperties, InputHTMLAttributes } from 'react';
import { Wind } from 'lucide-react';
import './FrostSlider.css';

type FrostSliderProps = {
  label: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  orientation?: 'horizontal' | 'vertical';
  onChange?: (value: number) => void;
} & Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'value' | 'min' | 'max' | 'step' | 'onChange'>;

export function FrostSlider({
  label,
  value,
  min = 0,
  max = 100,
  step = 1,
  orientation = 'horizontal',
  onChange,
  ...inputProps
}: FrostSliderProps) {
  const ratio = Math.min(1, Math.max(0, (value - min) / (max - min)));

  return (
    <label
      className={`frost-slider frost-slider--${orientation}`}
      style={
        {
          '--slider-percent': `${ratio * 100}%`,
          '--slider-inverse-percent': `${(1 - ratio) * 100}%`,
        } as CSSProperties
      }
    >
      <span className="frost-slider__label">
        <Wind size={18} strokeWidth={1.8} />
        {label}
      </span>
      <span className="frost-slider__rail">
        <span className="frost-slider__fill" />
        <span className="frost-slider__thumb" />
      </span>
      <input
        {...inputProps}
        className="frost-slider__input"
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        aria-orientation={orientation}
        onChange={(event) => onChange?.(Number(event.currentTarget.value))}
      />
    </label>
  );
}
