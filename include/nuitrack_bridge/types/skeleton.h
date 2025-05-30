#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector>

#include "nuitrack/types/Skeleton.h"

namespace nuitrack_bridge::skeleton {
    using JointType = tdv::nuitrack::JointType;
    using Joint = tdv::nuitrack::Joint;
    using Skeleton = tdv::nuitrack::Skeleton;

    int32_t getUserId(const Skeleton& skeleton);
    std::unique_ptr<std::vector<Joint>> getJoints(const Skeleton& skeleton); // New

    // --- Joint functions ---
    JointType getJointType(const Joint& joint); // New
    float getJointConfidence(const Joint& joint); // New

    // Real position
    float getJointRealX(const Joint& joint); // New
    float getJointRealY(const Joint& joint); // New
    float getJointRealZ(const Joint& joint); // New

    // Projection
    float getJointProjX(const Joint& joint); // New
    float getJointProjY(const Joint& joint); // New
    float getJointProjZ(const Joint& joint); // New

    // Orientation (as a flat 3x3 matrix)
    std::unique_ptr<std::vector<float>> getJointOrientationMatrix(const Joint& joint); // New

    void doNotUseMakeVectorElementAware(const std::vector<Skeleton>& data);
}