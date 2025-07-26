#pragma once
#include "rust/cxx.h"
#include <memory>

namespace tdv::nuitrack {
    class Nuitrack;
    class ColorSensor;
    class HandTracker;
    class SkeletonTracker;
    class DepthSensor;
    class UserTracker;
    class GestureRecognizer;
}

namespace nuitrack_bridge::core {
    using Nuitrack = tdv::nuitrack::Nuitrack;
    using ColorSensor = tdv::nuitrack::ColorSensor;
    using HandTracker = tdv::nuitrack::HandTracker;
    using SkeletonTracker = tdv::nuitrack::SkeletonTracker;
    using DepthSensor = tdv::nuitrack::DepthSensor;
    using UserTracker = tdv::nuitrack::UserTracker;
    using GestureRecognizer = tdv::nuitrack::GestureRecognizer;

    void init(rust::Str configPath);
    void run();
    void update();
    void waitUpdateColorSensor(const std::shared_ptr<ColorSensor>& colorSensor);
    void waitUpdateHandTracker(const std::shared_ptr<HandTracker>& handTracker);
    void waitUpdateSkeletonTracker(const std::shared_ptr<SkeletonTracker>& skeletonTracker);
    void waitUpdateDepthSensor(const std::shared_ptr<DepthSensor>& depthSensor);
    void waitUpdateUserTracker(const std::shared_ptr<UserTracker>& userTracker);
    void waitUpdateGestureRecognizer(const std::shared_ptr<GestureRecognizer>& gestureRecognizer);
    void release();

    void setConfigValue(rust::Str key, rust::Str value);
    rust::String getConfigValue(rust::Str key);
}