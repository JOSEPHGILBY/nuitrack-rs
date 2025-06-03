#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector>

#include "nuitrack/types/Skeleton.h"

namespace tdv::nuitrack {
    class SkeletonData;
}

namespace nuitrack_bridge::skeleton_data {
    using SkeletonData = tdv::nuitrack::SkeletonData;
    using Skeleton = tdv::nuitrack::Skeleton;

    int32_t getNumSkeletons(const SkeletonData& frame);

    uint64_t getTimestamp(const SkeletonData& frame);

    std::unique_ptr<std::vector<Skeleton>> getSkeletons(const SkeletonData& frame);
}