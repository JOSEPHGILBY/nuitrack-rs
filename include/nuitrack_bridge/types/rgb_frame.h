#pragma once

#include "rust/cxx.h"
#include <memory>
#include <cstdint>

namespace tdv::nuitrack {
    class RGBFrame;
    struct Color3;
}

namespace nuitrack_bridge::rgb_frame {
    using RGBFrame = tdv::nuitrack::RGBFrame;
    using NuitrackColor3 = tdv::nuitrack::Color3;
    struct Color3;

    int32_t getRows(const RGBFrame& frame);
    int32_t getCols(const RGBFrame& frame);
    uint64_t getID(const RGBFrame& frame);
    uint64_t getTimestamp(const RGBFrame& frame);

    rust::Slice<const Color3> getData(const RGBFrame& frame);
}