#include "PluginProcessor.h"
#include "PluginEditor.h"

namespace
{
float readParameter (const juce::AudioProcessorValueTreeState& parameters, std::string_view id)
{
    if (auto* value = parameters.getRawParameterValue (juce::String { id.data(), id.size() }))
        return value->load();

    jassertfalse;
    return 0.0f;
}
}

FrostVerbAudioProcessor::FrostVerbAudioProcessor()
    : AudioProcessor (BusesProperties()
          .withInput ("Input", juce::AudioChannelSet::stereo(), true)
          .withOutput ("Output", juce::AudioChannelSet::stereo(), true)),
      parameters (*this, nullptr, "FrostVerbParameters", createParameterLayout()),
      engine (frostverb_engine_create())
{
    jassert (frostverb_parameter_count() == frostverb::parameterSpecs.size());
}

FrostVerbAudioProcessor::~FrostVerbAudioProcessor() = default;

void FrostVerbAudioProcessor::EngineDeleter::operator() (FrostVerbFfiEngine* handle) const noexcept
{
    frostverb_engine_destroy (handle);
}

juce::AudioProcessorValueTreeState::ParameterLayout FrostVerbAudioProcessor::createParameterLayout()
{
    std::vector<std::unique_ptr<juce::RangedAudioParameter>> params;
    params.reserve (frostverb::parameterSpecs.size());

    for (const auto& spec : frostverb::parameterSpecs)
    {
        params.push_back (std::make_unique<juce::AudioParameterFloat>(
            juce::ParameterID { juce::String { spec.id.data(), spec.id.size() }, 1 },
            juce::String { spec.name.data(), spec.name.size() },
            juce::NormalisableRange<float> { 0.0f, 1.0f },
            spec.defaultValue));
    }

    return { params.begin(), params.end() };
}

void FrostVerbAudioProcessor::prepareToPlay (double sampleRate, int samplesPerBlock)
{
    frostverb_engine_prepare (engine.get(), static_cast<float> (sampleRate));
    interleaved.assign (static_cast<std::size_t> (std::max (1, samplesPerBlock)) * 2, 0.0f);
}

void FrostVerbAudioProcessor::releaseResources()
{
    frostverb_engine_reset (engine.get());
    interleaved.clear();
}

bool FrostVerbAudioProcessor::isBusesLayoutSupported (const BusesLayout& layouts) const
{
    return layouts.getMainInputChannelSet() == juce::AudioChannelSet::stereo()
        && layouts.getMainOutputChannelSet() == juce::AudioChannelSet::stereo();
}

void FrostVerbAudioProcessor::processBlock (juce::AudioBuffer<float>& buffer, juce::MidiBuffer&)
{
    juce::ScopedNoDenormals noDenormals;

    const auto frames = buffer.getNumSamples();
    const auto requiredSamples = static_cast<std::size_t> (frames) * 2;
    if (interleaved.size() < requiredSamples)
        interleaved.resize (requiredSamples, 0.0f);

    const auto* leftIn = buffer.getReadPointer (0);
    const auto* rightIn = buffer.getReadPointer (1);
    for (int i = 0; i < frames; ++i)
    {
        interleaved[static_cast<std::size_t> (i) * 2] = leftIn[i];
        interleaved[static_cast<std::size_t> (i) * 2 + 1] = rightIn[i];
    }

    frostverb_engine_process_interleaved (engine.get(), interleaved.data(), static_cast<std::size_t> (frames), readParams());

    auto* leftOut = buffer.getWritePointer (0);
    auto* rightOut = buffer.getWritePointer (1);
    for (int i = 0; i < frames; ++i)
    {
        leftOut[i] = interleaved[static_cast<std::size_t> (i) * 2];
        rightOut[i] = interleaved[static_cast<std::size_t> (i) * 2 + 1];
    }
}

juce::AudioProcessorEditor* FrostVerbAudioProcessor::createEditor()
{
    return new FrostVerbAudioProcessorEditor (*this);
}

void FrostVerbAudioProcessor::getStateInformation (juce::MemoryBlock& destData)
{
    if (auto xml = parameters.copyState().createXml())
        copyXmlToBinary (*xml, destData);
}

void FrostVerbAudioProcessor::setStateInformation (const void* data, int sizeInBytes)
{
    if (auto xml = getXmlFromBinary (data, sizeInBytes))
        parameters.replaceState (juce::ValueTree::fromXml (*xml));
}

FrostVerbFfiMeters FrostVerbAudioProcessor::getMeters() const
{
    return frostverb_engine_meters (engine.get());
}

FrostVerbFfiParams FrostVerbAudioProcessor::readParams() const
{
    return {
        readParameter (parameters, "coldness"),
        readParameter (parameters, "wind"),
        readParameter (parameters, "ice_size"),
        readParameter (parameters, "ancient_depth"),
        readParameter (parameters, "frozen_harmonics"),
        readParameter (parameters, "storm"),
        readParameter (parameters, "distance"),
        readParameter (parameters, "decay"),
        readParameter (parameters, "freeze"),
        readParameter (parameters, "mix"),
        readParameter (parameters, "input"),
        readParameter (parameters, "output"),
    };
}

juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new FrostVerbAudioProcessor();
}
