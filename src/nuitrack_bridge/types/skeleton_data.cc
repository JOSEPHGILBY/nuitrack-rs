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

    void doNotUseMakeSharedPtrAware(const std::shared_ptr<SkeletonData>& data) {
        // This function can be a no-op if it's just for CXX's type system.
        // Or it could do some light validation or logging if desired.
        (void)data; // Suppress unused parameter warning if it's a no-op
    }
}