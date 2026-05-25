#pragma once

#include <array>
#include <string_view>

namespace frostverb
{
struct ParameterSpec
{
    std::string_view id;
    std::string_view name;
    float defaultValue;
};

inline constexpr std::array<ParameterSpec, 12> parameterSpecs {{
    { "coldness", "Coldness", 0.62f },
    { "wind", "Wind", 0.18f },
    { "ice_size", "Ice Size", 0.55f },
    { "ancient_depth", "Ancient Depth", 0.48f },
    { "frozen_harmonics", "Frozen Harmonics", 0.24f },
    { "storm", "Storm", 0.20f },
    { "distance", "Distance", 0.42f },
    { "decay", "Decay", 0.58f },
    { "freeze", "Freeze", 0.0f },
    { "mix", "Mix", 0.34f },
    { "input", "Input", 0.75f },
    { "output", "Output", 0.75f },
}};
}
