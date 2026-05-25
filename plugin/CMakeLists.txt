cmake_minimum_required(VERSION 3.22)

project(FrostVerbJucePlugin VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

option(FROSTVERB_FETCH_JUCE "Fetch JUCE with CMake FetchContent when FROSTVERB_JUCE_DIR is not set" OFF)
set(FROSTVERB_JUCE_DIR "" CACHE PATH "Path to a local JUCE checkout")

if(FROSTVERB_JUCE_DIR)
    add_subdirectory("${FROSTVERB_JUCE_DIR}" JUCE)
elseif(FROSTVERB_FETCH_JUCE)
    include(FetchContent)
    FetchContent_Declare(
        JUCE
        GIT_REPOSITORY https://github.com/juce-framework/JUCE.git
        GIT_TAG 8.0.6
        GIT_SHALLOW TRUE)
    FetchContent_MakeAvailable(JUCE)
else()
    message(FATAL_ERROR "Set FROSTVERB_JUCE_DIR to a JUCE 8 checkout or enable FROSTVERB_FETCH_JUCE=ON.")
endif()

find_program(CARGO_EXECUTABLE cargo REQUIRED)

set(FROSTVERB_ROOT "${CMAKE_CURRENT_LIST_DIR}/..")
set(FROSTVERB_RUST_LIB "${FROSTVERB_ROOT}/target/release/${CMAKE_STATIC_LIBRARY_PREFIX}frostverb${CMAKE_STATIC_LIBRARY_SUFFIX}")

add_custom_target(FrostVerbRust
    COMMAND "${CARGO_EXECUTABLE}" build --release
    WORKING_DIRECTORY "${FROSTVERB_ROOT}"
    BYPRODUCTS "${FROSTVERB_RUST_LIB}"
    COMMENT "Building Rust DSP static library")

add_library(FrostVerbRustStatic STATIC IMPORTED GLOBAL)
set_target_properties(FrostVerbRustStatic PROPERTIES IMPORTED_LOCATION "${FROSTVERB_RUST_LIB}")
add_dependencies(FrostVerbRustStatic FrostVerbRust)

juce_add_plugin(FrostVerb
    COMPANY_NAME "Nursven"
    BUNDLE_ID "com.nursven.frostverb"
    PLUGIN_MANUFACTURER_CODE Nurs
    PLUGIN_CODE Frvb
    FORMATS VST3 AU Standalone
    IS_SYNTH FALSE
    NEEDS_MIDI_INPUT FALSE
    NEEDS_MIDI_OUTPUT FALSE
    IS_MIDI_EFFECT FALSE
    EDITOR_WANTS_KEYBOARD_FOCUS TRUE
    COPY_PLUGIN_AFTER_BUILD FALSE
    PRODUCT_NAME "Frost Verb")

juce_add_binary_data(FrostVerbWebAssets
    SOURCES
        web/index.html
        web/styles.css
        web/app.js)

target_sources(FrostVerb
    PRIVATE
        Source/PluginProcessor.cpp
        Source/PluginEditor.cpp)

if(APPLE)
    target_sources(FrostVerb PRIVATE Source/WebViewDebug_mac.mm)
else()
    target_sources(FrostVerb PRIVATE Source/WebViewDebug.cpp)
endif()

target_include_directories(FrostVerb
    PRIVATE
        Source
        include)

target_compile_definitions(FrostVerb
    PRIVATE
        JUCE_WEB_BROWSER=1
        JUCE_USE_WIN_WEBVIEW2_WITH_STATIC_LINKING=1)

target_link_libraries(FrostVerb
    PRIVATE
        FrostVerbRustStatic
        FrostVerbWebAssets
        juce::juce_audio_utils
        juce::juce_dsp
        juce::juce_gui_extra
    PUBLIC
        juce::juce_recommended_config_flags
        juce::juce_recommended_lto_flags
        juce::juce_recommended_warning_flags)
