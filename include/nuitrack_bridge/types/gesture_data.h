#pragma once

#include "rust/cxx.h"
#include <memory>
#include <cstdint>
#include <vector>

#include "nuitrack/types/GestureData.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/gesture.rs.h"

namespace nuitrack_bridge::gesture_data {
    // Create type aliases for convenience
    using GestureData = tdv::nuitrack::GestureData;
    using UserStateData = tdv::nuitrack::UserStateData;
    using UserGesturesStateData = tdv::nuitrack::UserGesturesStateData;

    using Gesture = nuitrack_bridge::gesture::Gesture;
    using UserState = nuitrack_bridge::gesture::UserState;
    using UserGesturesState = nuitrack_bridge::gesture::UserGesturesState;

    // --- GestureData Functions ---
    uint64_t getGestureDataTimestamp(const GestureData& data);
    int32_t getGestureDataNumGestures(const GestureData& data);
    rust::Slice<const Gesture> getGestureDataGestures(const GestureData& data);

    // --- UserStateData Functions ---
    uint64_t getUserStateDataTimestamp(const UserStateData& data);
    int32_t getUserStateDataNumUserStates(const UserStateData& data);
    rust::Slice<const UserState> getUserStateDataUserStates(const UserStateData& data);

    // --- UserGesturesStateData Functions ---
    uint64_t getUserGesturesStateDataTimestamp(const UserGesturesStateData& data);
    int32_t getUserGesturesStateDataNumUsers(const UserGesturesStateData& data);
    std::unique_ptr<std::vector<UserGesturesState>> getUserGesturesStateDataUserGesturesStates(const UserGesturesStateData& data);
}