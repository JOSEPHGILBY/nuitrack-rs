#include "nuitrack_bridge/types/user_frame.h"
#include "nuitrack/types/UserFrame.h"

namespace nuitrack_bridge::user_frame {

    std::unique_ptr<std::vector<User>> getUsers(const UserFrame& frame) {
        // Get the original vector from the Nuitrack SDK
        const auto sdk_users = frame.getUsers();
        
        // Create a new vector to hold the bridge-compatible User structs
        auto bridge_users = std::make_unique<std::vector<User>>();
        bridge_users->reserve(sdk_users.size());

        // Since the Rust-defined `User` has an identical memory layout to the SDK's `tdv::nuitrack::User`,
        // we can safely cast and copy each element.
        for (const auto& sdk_user : sdk_users) {
            bridge_users->push_back(*reinterpret_cast<const User*>(&sdk_user));
        }
        
        return bridge_users;
    }

    int32_t getRows(const UserFrame& frame) {
        return static_cast<int32_t>(frame.getRows());
    }

    int32_t getCols(const UserFrame& frame) {
        return static_cast<int32_t>(frame.getCols());
    }

    uint64_t getID(const UserFrame& frame) {
        return frame.getID();
    }

    rust::Slice<const uint16_t> getData(const UserFrame& frame) {
        return rust::Slice<const uint16_t>{
            frame.getData(),
            static_cast<size_t>(frame.getRows() * frame.getCols())
        };
    }

    uint64_t getTimestamp(const UserFrame& frame) {
        return frame.getTimestamp();
    }

    Vector3 getFloor(const UserFrame& frame) {
        auto sdk_vector = frame.getFloor();
        // Cast the SDK's Vector3 to the bridge's layout-compatible Vector3
        return *reinterpret_cast<Vector3*>(&sdk_vector);
    }

    Vector3 getFloorNormal(const UserFrame& frame) {
        auto sdk_vector = frame.getFloorNormal();
        // Cast the SDK's Vector3 to the bridge's layout-compatible Vector3
        return *reinterpret_cast<Vector3*>(&sdk_vector);
    }

} // namespace nuitrack_bridge::user_frame