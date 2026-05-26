import type { Meta, StoryObj } from '@storybook/react';
import { FrostSlider } from './FrostSlider';

const meta = {
  title: 'Frostverb/Controls/FrostSlider',
  component: FrostSlider,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  args: {
    label: 'Wind',
    value: 55,
  },
} satisfies Meta<typeof FrostSlider>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Horizontal: Story = {};

export const Vertical: Story = {
  args: {
    label: 'Ancient Depth',
    value: 72,
    orientation: 'vertical',
  },
};
