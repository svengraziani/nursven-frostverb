import type { Meta, StoryObj } from '@storybook/react';
import { FrostPluginPanel } from './FrostPluginPanel';

const meta = {
  title: 'Frostverb/Compositions/FrostPluginPanel',
  component: FrostPluginPanel,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof FrostPluginPanel>;

export default meta;
type Story = StoryObj<typeof meta>;

export const PluginSurface: Story = {};
