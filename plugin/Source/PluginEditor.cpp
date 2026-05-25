#include "PluginEditor.h"

#include "BinaryData.h"
#include "ParameterIds.h"

namespace
{
std::optional<juce::WebBrowserComponent::Resource> findWebResource (const juce::String& path)
{
    auto normalised = path.fromFirstOccurrenceOf ("/", false, false);
    if (normalised.isEmpty())
        normalised = "index.html";

    auto addResource = [&] (const char* data, int size, const char* mime) {
        const auto* bytes = reinterpret_cast<const std::byte*> (data);
        return juce::WebBrowserComponent::Resource {
            std::vector<std::byte> { bytes, bytes + size },
            juce::String { mime },
        };
    };

    if (normalised == "index.html") return addResource (BinaryData::index_html, BinaryData::index_htmlSize, "text/html");
    if (normalised == "styles.css") return addResource (BinaryData::styles_css, BinaryData::styles_cssSize, "text/css");
    if (normalised == "app.js") return addResource (BinaryData::app_js, BinaryData::app_jsSize, "text/javascript");

    return std::nullopt;
}
}

FrostVerbAudioProcessorEditor::FrostVerbAudioProcessorEditor (FrostVerbAudioProcessor& p)
    : AudioProcessorEditor (&p),
      audioProcessor (p),
      browser (createBrowserOptions())
{
    setSize (1000, 620);
    addAndMakeVisible (browser);
    browser.goToURL (juce::WebBrowserComponent::getResourceProviderRoot());
    startTimerHz (30);
}

void FrostVerbAudioProcessorEditor::resized()
{
    browser.setBounds (getLocalBounds());
}

void FrostVerbAudioProcessorEditor::timerCallback()
{
    emitInitialParameterState();

    const auto meters = audioProcessor.getMeters();
    browser.emitEventIfBrowserIsVisible ("meters", juce::var { juce::Array<juce::var> {
        meters.input_peak,
        meters.output_peak,
        meters.wet_peak,
        meters.limiter_gain_reduction_db,
    } });
}

void FrostVerbAudioProcessorEditor::emitInitialParameterState()
{
    auto values = juce::Array<juce::var> {};
    for (const auto& spec : frostverb::parameterSpecs)
    {
        if (auto* value = audioProcessor.getParameters().getRawParameterValue (juce::String { spec.id.data(), spec.id.size() }))
        {
            auto object = new juce::DynamicObject();
            object->setProperty ("id", juce::String { spec.id.data(), spec.id.size() });
            object->setProperty ("value", value->load());
            values.add (juce::var { object });
        }
    }

    browser.emitEventIfBrowserIsVisible ("parameterSnapshot", juce::var { values });
}

juce::WebBrowserComponent::Options FrostVerbAudioProcessorEditor::createBrowserOptions()
{
    auto options = juce::WebBrowserComponent::Options {}
        .withNativeIntegrationEnabled()
        .withResourceProvider ([] (const juce::String& path) { return findWebResource (path); })
        .withNativeFunction ("setParameter", [this] (const auto& args, auto complete) {
            if (args.size() >= 2)
            {
                const auto id = args[0].toString();
                const auto value = static_cast<float> (args[1]);
                if (auto* parameter = audioProcessor.getParameters().getParameter (id))
                    parameter->setValueNotifyingHost (juce::jlimit (0.0f, 1.0f, value));
            }
            complete ("ok");
        });

   #if JUCE_WINDOWS
    options = options.withBackend (juce::WebBrowserComponent::Options::Backend::webview2)
        .withWinWebView2Options (juce::WebBrowserComponent::Options::WinWebView2 {}
            .withUserDataFolder (juce::File::getSpecialLocation (juce::File::tempDirectory)));
   #endif

    return options;
}
