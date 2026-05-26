import type { Meta, StoryObj } from '@storybook/react';
import { FrostKnob } from './FrostKnob';

const meta = {
  title: 'Frostverb/Controls/FrostKnob',
  component: FrostKnob,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  args: {
    label: 'Ice Resonance',
    value: 64,
    min: 0,
    max: 100,
    unit: '%',
    variant: 'iceKnob',
  },
  argTypes: {
    variant: {
      control: 'select',
      options: ['iceKnob', 'runeKnob', 'crystalKnob', 'woodKnob'],
    },
  },
} satisfies Meta<typeof FrostKnob>;

export default meta;
type Story = StoryObj<typeof meta>;

export const IceResonance: Story = {};

export const RuneStorm: Story = {
  args: {
    label: 'Storm',
    value: 38,
    variant: 'runeKnob',
  },
};

export const CrystalFreeze: Story = {
  args: {
    label: 'Spectral Freeze',
    value: 82,
    variant: 'crystalKnob',
  },
};
