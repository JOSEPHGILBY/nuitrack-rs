#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector>

#include "nuitrack/types/Skeleton.h"

namespace nuitrack_bridge::skeleton {
    //using JointType = tdv::nuitrack::JointType;
    //using NuitrackJoint = tdv::nuitrack::Joint;
    using Skeleton = tdv::nuitrack::Skeleton;
    struct Joint;
    

    int32_t getUserID(const Skeleton& skeleton);
    rust::Slice<const Joint> getJoints(const Skeleton& skeleton);
}