#pragma once

#include "rust/cxx.h"
#include <memory>
#include <cstdint>

#include "nuitrack-rs/src/nuitrack_bridge/types/output_mode.rs.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/vector3.rs.h"

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class DepthSensor;
    class DepthFrame;
    struct Vector3;
    struct OutputMode;
}

namespace nuitrack_bridge::depth_sensor {

    using DepthSensor = tdv::nuitrack::DepthSensor;
    using DepthFrame = tdv::nuitrack::DepthFrame;
    using CPPVector3 = tdv::nuitrack::Vector3;
    using CPPOutputMode = tdv::nuitrack::OutputMode;

    // CXX-bridged types
    using OutputMode = nuitrack_bridge::output_mode::OutputMode;
    using Vector3 = nuitrack_bridge::vector3::Vector3;
    //struct Vector3; // Definition will be in the .rs file

    using c_void = void; // For opaque pointer from Rust

    std::shared_ptr<DepthSensor> createDepthSensor();

    uint64_t connectOnNewFrameForAsync(
        const std::shared_ptr<DepthSensor>& sensor,
        void* depthFrameSender // Opaque pointer to Rust sender
    );

    void disconnectOnNewFrame(
        const std::shared_ptr<DepthSensor>& sensor,
        uint64_t handlerId
    );
    
    std::shared_ptr<DepthFrame> getDepthFrame(const std::shared_ptr<DepthSensor>& sensor);
    OutputMode getOutputMode(const std::shared_ptr<DepthSensor>& sensor);
    bool isMirror(const std::shared_ptr<DepthSensor>& sensor);
    void setMirror(const std::shared_ptr<DepthSensor>& sensor, bool mirror);
    Vector3 convertProjToRealCoords(const std::shared_ptr<DepthSensor>& sensor, const Vector3& p);
    Vector3 convertRealToProjCoords(const std::shared_ptr<DepthSensor>& sensor, const Vector3& p);
    uint64_t getSensorTimestamp(const std::shared_ptr<DepthSensor>& sensor);
    bool canUpdate(const std::shared_ptr<DepthSensor>& sensor);

} // namespace nuitrack_bridge::depth_sensor

// Extern "C" function to be called from C++ lambda, implemented by Rust/CXX
extern "C" {
    void rust_depth_sensor_depth_frame_dispatcher_async(
        std::shared_ptr<tdv::nuitrack::DepthFrame>& frame, // Pass by non-const reference
        void* depthFrameSender
    );
}