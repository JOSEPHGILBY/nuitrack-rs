#include "nuitrack_bridge/types/hand.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/hand.rs.h"

namespace nuitrack_bridge::hand {


    std::shared_ptr<Hand> alias_hand_ptr(const std::shared_ptr<NuitrackHand>& sdk_hand_ptr) {
        if (!sdk_hand_ptr) {
            return nullptr;
        }

        NuitrackHand* raw_sdk_ptr = sdk_hand_ptr.get();
        Hand* casted_ptr = reinterpret_cast<Hand*>(raw_sdk_ptr);
        return std::shared_ptr<Hand>(sdk_hand_ptr, casted_ptr);
    }

    // --- UserHands functions ---
    int32_t getUserHandsUserId(const UserHands& userHands) {
        return static_cast<int32_t>(userHands.userId);
    }

    std::shared_ptr<Hand> getUserHandsLeftHand(const UserHands& userHands) {
        return alias_hand_ptr(userHands.leftHand);
    }

    std::shared_ptr<Hand> getUserHandsRightHand(const UserHands& userHands) {
        return alias_hand_ptr(userHands.rightHand);
    }
} 