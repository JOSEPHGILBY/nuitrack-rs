#include "nuitrack_bridge/types/skeleton.h"

namespace nuitrack_bridge::skeleton {

    int32_t getUserId(const Skeleton& skeleton) {
        return static_cast<int32_t>(skeleton.id);
    }

    std::unique_ptr<std::vector<Joint>> getJoints(const Skeleton& skeleton) {
        // skeleton.joints is std::vector<tdv::nuitrack::Joint>
        // Create a new vector on the heap, copying the contents.
        return std::make_unique<std::vector<Joint>>(skeleton.joints);
    }

    // --- Joint function implementations ---

    JointType getJointType(const Joint& joint) {
        return joint.type;
    }

    float getJointConfidence(const Joint& joint) {
        return joint.confidence;
    }

    // Real position
    float getJointRealX(const Joint& joint) {
        return joint.real.x;
    }
    float getJointRealY(const Joint& joint) {
        return joint.real.y;
    }
    float getJointRealZ(const Joint& joint) {
        return joint.real.z;
    }

    // Projection
    float getJointProjX(const Joint& joint) {
        return joint.proj.x;
    }
    float getJointProjY(const Joint& joint) {
        return joint.proj.y;
    }
    float getJointProjZ(const Joint& joint) {
        return joint.proj.z;
    }

    // Orientation (as a flat 3x3 matrix)
    std::unique_ptr<std::vector<float>> getJointOrientationMatrix(const Joint& joint) {
        auto matrix_vec = std::make_unique<std::vector<float>>();
        matrix_vec->reserve(9);
        for (int i = 0; i < 9; ++i) {
            matrix_vec->push_back(joint.orient.matrix[i]);
        }
        return matrix_vec;
    }

    void doNotUseMakeVectorElementAware(const std::vector<Skeleton>& data) {
        // This function can be a no-op if it's just for CXX's type system.
        // Or it could do some light validation or logging if desired.
        (void)data; // Suppress unused parameter warning if it's a no-op
    }

}