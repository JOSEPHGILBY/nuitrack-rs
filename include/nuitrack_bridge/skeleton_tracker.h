#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector> // For std::vector

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class SkeletonTracker;
}

namespace nuitrack_bridge::skeleton_tracker {

    using SkeletonTracker = tdv::nuitrack::SkeletonTracker;

    std::shared_ptr<SkeletonTracker> create_skeleton_tracker();

}