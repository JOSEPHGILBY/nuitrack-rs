#pragma once
#include "rust/cxx.h"
#include <memory>

namespace tdv::nuitrack {
    class Nuitrack;
    class ColorSensor;
    class HandTracker;
    class SkeletonTracker;
}

namespace nuitrack_bridge::core {
    using Nuitrack = tdv::nuitrack::Nuitrack;
    using ColorSensor = tdv::nuitrack::ColorSensor;
    using HandTracker = tdv::nuitrack::HandTracker;
    using SkeletonTracker = tdv::nuitrack::SkeletonTracker;

    void init(rust::Str configPath);
    void run();
    void update();
    void waitUpdateColorSensor(const std::shared_ptr<ColorSensor>& colorSensor);
    void waitUpdateHandTracker(const std::shared_ptr<HandTracker>& handTracker);
    void waitUpdateSkeletonTracker(const std::shared_ptr<SkeletonTracker>& skeletonTracker);
    void release();
}