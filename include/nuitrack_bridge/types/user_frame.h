#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector>

#include "nuitrack-rs/src/nuitrack_bridge/types/user.rs.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/vector3.rs.h"

namespace tdv::nuitrack {
    class UserFrame;
}

namespace nuitrack_bridge::user_frame {
    // Create type aliases for convenience
    using UserFrame = tdv::nuitrack::UserFrame;
    using User = nuitrack_bridge::user::User;
    using Vector3 = nuitrack_bridge::vector3::Vector3;

    // --- Function Declarations ---

    std::unique_ptr<std::vector<User>> getUsers(const UserFrame& frame);
    int32_t getRows(const UserFrame& frame);
    int32_t getCols(const UserFrame& frame);
    uint64_t getID(const UserFrame& frame);
    rust::Slice<const uint16_t> getData(const UserFrame& frame);
    uint64_t getTimestamp(const UserFrame& frame);
    Vector3 getFloor(const UserFrame& frame);
    Vector3 getFloorNormal(const UserFrame& frame);
}