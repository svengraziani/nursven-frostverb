#pragma once

#include <cstddef>

extern "C"
{
struct FrostVerbFfiParams
{
    float coldness;
    float wind;
    float ice_size;
    float ancient_depth;
    float frozen_harmonics;
    float storm;
    float distance;
    float decay;
    float freeze;
    float mix;
    float input;
    float output;
};

struct FrostVerbFfiMeters
{
    float input_peak;
    float output_peak;
    float wet_peak;
    float limiter_gain_reduction_db;
};

struct FrostVerbFfiEngine;

std::size_t frostverb_parameter_count();
FrostVerbFfiParams frostverb_default_params();
FrostVerbFfiEngine* frostverb_engine_create();
void frostverb_engine_destroy(FrostVerbFfiEngine* handle);
void frostverb_engine_prepare(FrostVerbFfiEngine* handle, float sample_rate);
void frostverb_engine_reset(FrostVerbFfiEngine* handle);
void frostverb_engine_process_interleaved(
    FrostVerbFfiEngine* handle,
    float* buffer,
    std::size_t frame_count,
    FrostVerbFfiParams params);
FrostVerbFfiMeters frostverb_engine_meters(const FrostVerbFfiEngine* handle);
}
