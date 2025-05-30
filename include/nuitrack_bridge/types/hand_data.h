#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector>

#include "nuitrack/types/Hand.h"

namespace tdv::nuitrack {
    class HandTrackerData;
    using HandData = HandTrackerData;
}

namespace nuitrack_bridge::hand_data {

    using HandData = tdv::nuitrack::HandData;
    using UserHands = tdv::nuitrack::UserHands;

    // --- HandTrackerData functions ---
    uint64_t getTimestamp(const HandData& data);
    int32_t getNumUsers(const HandData& data);
    std::unique_ptr<std::vector<UserHands>> getUsersHands(const HandData& data);

    void doNotUseMakeHandTrackerDataSharedPtrAware(const std::shared_ptr<HandData>& data);

}