#include "nuitrack_bridge/types/hand_data.h"
#include "nuitrack/types/HandTrackerData.h"
#include "nuitrack/types/Hand.h"           

namespace nuitrack_bridge::hand_data {

    uint64_t getTimestamp(const HandData& data) {
        // Assuming Nuitrack methods can throw, wrap in try-catch if necessary,
        // similar to skeleton_tracker.cc for robustness. For brevity, not shown here
        // but should be considered for production code.
        return data.getTimestamp();
    }

    int32_t getNumUsers(const HandData& data) {
        return static_cast<int32_t>(data.getNumUsers());
    }

    std::unique_ptr<std::vector<UserHands>> getUsersHands(const HandData& data) {
        return std::make_unique<std::vector<UserHands>>(data.getUsersHands());
    }

}