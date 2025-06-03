#pragma once

#include "rust/cxx.h"
#include <memory>
#include <cstdint>

namespace tdv::nuitrack {
    struct Intrinsics;
    struct OutputMode;
}

namespace nuitrack_bridge::rgb_frame {
    using CPPIntrinsics = tdv::nuitrack::Intrinsics;
    using CPPOutputMode = tdv::nuitrack::OutputMode;
    struct Intrinsics;
    struct OutputMode;
}