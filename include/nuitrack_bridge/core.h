#pragma once
#include "rust/cxx.h"
#include <memory>

namespace tdv::nuitrack {
    class HandTracker;
    class SkeletonTracker;
}

namespace nuitrack_bridge::core {
    void init(rust::Str config_path_str);
    void run();
    void update();
    void waitUpdateHandTracker(const std::shared_ptr<tdv::nuitrack::HandTracker>& hand_tracker_ptr);
    void waitUpdateSkeletonTracker(const std::shared_ptr<tdv::nuitrack::SkeletonTracker>& skeleton_tracker_ptr);
    void release();
}