#include "nuitrack_bridge/types/skeleton_data.h"
#include "nuitrack/types/SkeletonData.h"

namespace nuitrack_bridge::skeleton_data {

    int32_t getNumSkeletons(const SkeletonData& frame) {
        return static_cast<int32_t>(frame.getNumSkeletons());
    }

    std::unique_ptr<std::vector<Skeleton>> getSkeletons(const SkeletonData& frame) {
        return std::make_unique<std::vector<Skeleton>>(frame.getSkeletons());
    }

    uint64_t getTimestamp(const SkeletonData& frame) {
        return frame.getTimestamp();
    }

}