import type { Meta, StoryObj } from '@storybook/react';
import { FrostMeter } from './FrostMeter';

const meta = {
  title: 'Frostverb/Controls/FrostMeter',
  component: FrostMeter,
  parameters: {
    layout: 'centered',
  },
} satisfies Meta<typeof FrostMeter>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Stereo: Story = {
  args: {
    label: 'Return',
    channels: [
      { label: 'L', value: 44 },
      { label: 'R', value: 49 },
    ],
    width: 560,
  },
};

export const Single: Story = {
  args: {
    label: 'Freeze Level',
    value: 76,
    width: 520,
  },
};
