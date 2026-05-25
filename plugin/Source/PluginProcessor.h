#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_audio_utils/juce_audio_utils.h>
#include <memory>
#include <vector>

#include "ParameterIds.h"
#include "frostverb/frostverb_ffi.h"

class FrostVerbAudioProcessor final : public juce::AudioProcessor
{
public:
    FrostVerbAudioProcessor();
    ~FrostVerbAudioProcessor() override;

    void prepareToPlay (double sampleRate, int samplesPerBlock) override;
    void releaseResources() override;
    bool isBusesLayoutSupported (const BusesLayout& layouts) const override;
    void processBlock (juce::AudioBuffer<float>& buffer, juce::MidiBuffer& midiMessages) override;

    juce::AudioProcessorEditor* createEditor() override;
    bool hasEditor() const override { return true; }

    const juce::String getName() const override { return JucePlugin_Name; }
    bool acceptsMidi() const override { return false; }
    bool producesMidi() const override { return false; }
    bool isMidiEffect() const override { return false; }
    double getTailLengthSeconds() const override { return 12.0; }

    int getNumPrograms() override { return 1; }
    int getCurrentProgram() override { return 0; }
    void setCurrentProgram (int) override {}
    const juce::String getProgramName (int) override { return {}; }
    void changeProgramName (int, const juce::String&) override {}

    void getStateInformation (juce::MemoryBlock& destData) override;
    void setStateInformation (const void* data, int sizeInBytes) override;

    FrostVerbFfiMeters getMeters() const;
    juce::AudioProcessorValueTreeState& getParameters() { return parameters; }

private:
    static juce::AudioProcessorValueTreeState::ParameterLayout createParameterLayout();
    FrostVerbFfiParams readParams() const;

    struct EngineDeleter
    {
        void operator() (FrostVerbFfiEngine* engine) const noexcept;
    };

    juce::AudioProcessorValueTreeState parameters;
    std::unique_ptr<FrostVerbFfiEngine, EngineDeleter> engine;
    std::vector<float> interleaved;

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (FrostVerbAudioProcessor)
};
