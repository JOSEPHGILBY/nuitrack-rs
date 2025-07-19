#pragma once

#include "rust/cxx.h"
#include <memory>
#include <cstdint>

namespace tdv::nuitrack {
    class DepthFrame;
}

namespace nuitrack_bridge::depth_frame {
    using DepthFrame = tdv::nuitrack::DepthFrame;

    int32_t getRows(const DepthFrame& frame);
    int32_t getCols(const DepthFrame& frame);
    uint64_t getID(const DepthFrame& frame);
    uint64_t getTimestamp(const DepthFrame& frame);

    rust::Slice<const uint16_t> getData(const DepthFrame& frame);
}