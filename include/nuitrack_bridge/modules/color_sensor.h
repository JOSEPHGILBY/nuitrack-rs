#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector> // Though not directly used, good for consistency

#include "nuitrack-rs/src/nuitrack_bridge/types/output_mode.rs.h"

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class ColorSensor;
    class RGBFrame;
    struct OutputMode; // Assuming OutputMode is a struct
}

namespace nuitrack_bridge::color_sensor {

    using ColorSensor = tdv::nuitrack::ColorSensor;
    using RGBFrame = tdv::nuitrack::RGBFrame;
    using CPPOutputMode = tdv::nuitrack::OutputMode;

    using OutputMode = nuitrack_bridge::output_mode::OutputMode;

    using c_void = void; // For opaque pointer from Rust

    std::shared_ptr<ColorSensor> createColorSensor();

    uint64_t connectOnNewFrameForAsync(
        const std::shared_ptr<ColorSensor>& sensor,
        void* rgbFrameSender // Opaque pointer to Rust sender
    );

    void disconnectOnNewFrame(
        const std::shared_ptr<ColorSensor>& sensor,
        uint64_t handlerId
    );

    OutputMode getOutputMode(const std::shared_ptr<ColorSensor>& sensor);

    std::shared_ptr<RGBFrame> getColorFrame(const std::shared_ptr<ColorSensor>& sensor);

    // Renamed to avoid potential conflicts if RGBFrame also has getTimestamp in FFI
    uint64_t getSensorTimestamp(const std::shared_ptr<ColorSensor>& sensor);

    bool canUpdate(const std::shared_ptr<ColorSensor>& sensor);

} // namespace nuitrack_bridge::color_sensor

// Extern "C" function to be called from C++ lambda, implemented by Rust/CXX
extern "C" {
    void rust_color_sensor_rgb_frame_dispatcher_async(
        std::shared_ptr<tdv::nuitrack::RGBFrame>& frame, // Pass by non-const reference
        void* rgbFrameSender
    );
}