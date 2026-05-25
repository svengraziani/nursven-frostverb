#pragma once

#include <juce_gui_extra/juce_gui_extra.h>
#include <cstddef>
#include <memory>
#include <vector>

#include "PluginProcessor.h"

class FrostVerbAudioProcessorEditor final
    : public juce::AudioProcessorEditor,
      private juce::Timer
{
public:
    explicit FrostVerbAudioProcessorEditor (FrostVerbAudioProcessor&);
    ~FrostVerbAudioProcessorEditor() override = default;

    void resized() override;

private:
    void timerCallback() override;
    void enableWebInspectorIfAvailable();
    void emitInitialParameterState();
    juce::WebBrowserComponent::Options createBrowserOptions();

    FrostVerbAudioProcessor& audioProcessor;
    juce::WebBrowserComponent browser;
    bool webInspectorEnabled = false;

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (FrostVerbAudioProcessorEditor)
};
