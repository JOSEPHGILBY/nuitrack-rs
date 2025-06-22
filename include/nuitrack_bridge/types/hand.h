#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector>

#include "nuitrack/types/Hand.h"

namespace nuitrack_bridge::hand {
    using NuitrackHand = tdv::nuitrack::Hand;
    struct Hand;
    using UserHands = tdv::nuitrack::UserHands;

    // // --- Hand functions ---
    // float getHandX(const Hand& hand);
    // float getHandY(const Hand& hand);
    // bool getHandClick(const Hand& hand);
    // int32_t getHandPressure(const Hand& hand);
    // float getHandXReal(const Hand& hand);
    // float getHandYReal(const Hand& hand);
    // float getHandZReal(const Hand& hand);

    // --- UserHands functions ---
    int32_t getUserHandsUserId(const UserHands& userHands);
    std::shared_ptr<Hand> getUserHandsLeftHand(const UserHands& userHands);
    std::shared_ptr<Hand> getUserHandsRightHand(const UserHands& userHands);

} // namespace nuitrack_bridge::hand