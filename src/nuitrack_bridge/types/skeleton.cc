#include "nuitrack_bridge/types/skeleton.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/skeleton.rs.h"

namespace nuitrack_bridge::skeleton {

    int32_t getUserID(const Skeleton& skeleton) {
        return static_cast<int32_t>(skeleton.id);
    }

    rust::Slice<const Joint> getJoints(const Skeleton& skeleton) {
        const auto& joint_vec = skeleton.joints;
        return rust::Slice<const Joint>{
            reinterpret_cast<const Joint*>(joint_vec.data()),
            joint_vec.size()
        };
    }

}