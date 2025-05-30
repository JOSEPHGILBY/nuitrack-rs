#pragma once
#include "rust/cxx.h"
#include <memory>

namespace tdv::nuitrack {
    class Nuitrack;
    class HandTracker;
    class SkeletonTracker;
}

namespace nuitrack_bridge::core {
    using Nuitrack = tdv::nuitrack::Nuitrack;
    using HandTracker = tdv::nuitrack::HandTracker;
    using SkeletonTracker = tdv::nuitrack::SkeletonTracker;

    void init(rust::Str configPath);
    void run();
    void update();
    void waitUpdateHandTracker(const std::shared_ptr<HandTracker>& handTracker);
    void waitUpdateSkeletonTracker(const std::shared_ptr<SkeletonTracker>& skeletonTracker);
    void release();
}