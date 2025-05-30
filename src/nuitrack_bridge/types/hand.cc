#include "nuitrack_bridge/types/hand.h"
#include "nuitrack/types/Hand.h" 

namespace nuitrack_bridge::hand {

    // --- Hand functions ---
    float getHandX(const Hand& hand) {
        return hand.x;
    }
    float getHandY(const Hand& hand) {
        return hand.y;
    }
    bool getHandClick(const Hand& hand) {
        return hand.click;
    }
    int getHandPressure(const Hand& hand) {
        return hand.pressure;
    }
    float getHandXReal(const Hand& hand) {
        return hand.xReal;
    }
    float getHandYReal(const Hand& hand) {
        return hand.yReal;
    }
    float getHandZReal(const Hand& hand) {
        return hand.zReal;
    }

    // --- UserHands functions ---
    int32_t getUserHandsUserId(const UserHands& userHands) {
        return static_cast<int32_t>(userHands.userId);
    }

    std::shared_ptr<Hand> getUserHandsLeftHand(const UserHands& userHands) {
        return userHands.leftHand;
    }

    std::shared_ptr<Hand> getUserHandsRightHand(const UserHands& userHands) {
        return userHands.rightHand;
    }
    
    void doNotUseMakeUserHandsVectorElementAware(const std::vector<UserHands>& /*vec*/) {
        // No-op
    }

    void doNotUseMakeHandSharedPtrAware(const std::shared_ptr<Hand>& /*handPtr*/) {
        // No-op
    }

} 