#include "nuitrack_bridge/hand_tracker.h"
#include "nuitrack/modules/HandTracker.h"
#include "nuitrack/types/HandTrackerData.h" // Includes Hand.h and UserHands
#include "nuitrack/Nuitrack.h"
#include <stdexcept>
#include <string>

namespace nuitrack_bridge::hand_tracker {

    // --- HandTracker Methods ---
    std::shared_ptr<HandTracker> create_hand_tracker() {
        try {
            return tdv::nuitrack::HandTracker::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack HandTracker::create failed: ") + e.what());
        } catch (const std::exception& e) { 
            throw;
        } catch (...) { 
            throw std::runtime_error("Unknown exception during Nuitrack HandTracker::create"); 
        }
    }

    uint64_t connect_on_update_wrapper(
        const std::shared_ptr<HandTracker>& tracker,
        rust::Fn<void(std::shared_ptr<BridgedHandTrackerData>)> callback
    ) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in connect_on_update_wrapper");
        }
        try {
            return tracker->connectOnUpdate(
                [callback](tdv::nuitrack::HandTrackerData::Ptr original_nuitrack_data) {
                    auto bridged_data_wrapper = std::make_shared<BridgedHandTrackerData>(std::move(original_nuitrack_data));
                    callback(bridged_data_wrapper);
            });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack HandTracker connectOnUpdate failed: ") + e.what());
        } catch (const std::exception& e) { 
            throw;
        } catch (...) { 
            throw std::runtime_error("Unknown exception during Nuitrack HandTracker connectOnUpdate"); 
        }
    }

    uint64_t connect_on_update_with_user_data(
        const std::shared_ptr<tdv::nuitrack::HandTracker>& tracker,
        void* user_data // This will be the Rust MPSC sender passed from Rust
    ) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in connect_on_update_with_user_data");
        }
        try {
            return tracker->connectOnUpdate(
                [user_data](tdv::nuitrack::HandTrackerData::Ptr original_nuitrack_data) {
                    // Wrap the Nuitrack data in our bridge struct
                    auto bridged_data_wrapper = std::make_shared<BridgedHandTrackerData>(original_nuitrack_data);
                    // Call the Rust dispatcher function, passing the wrapped data and user_data
                    rust_hand_tracker_callback_dispatcher(bridged_data_wrapper, user_data);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack HandTracker connectOnUpdate (with user_data) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw;
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack HandTracker connectOnUpdate (with user_data)");
        }
    }

    uint64_t connect_on_update_for_blocking(
        const std::shared_ptr<HandTracker>& tracker,
        void* user_data
    ) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in connect_on_update_with_user_data");
        }
        try {
            return tracker->connectOnUpdate(
                [user_data](tdv::nuitrack::HandTrackerData::Ptr original_nuitrack_data) {
                    auto bridged_data_wrapper = std::make_shared<BridgedHandTrackerData>(original_nuitrack_data);
                    rust_blocking_hand_tracker_trampoline(bridged_data_wrapper, user_data);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack HandTracker connectOnUpdate (with user_data) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw;
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack HandTracker connectOnUpdate (with user_data)");
        }
    }

    void disconnect_on_update_wrapper(
        const std::shared_ptr<HandTracker>& tracker,
        uint64_t handler_id
    ) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in disconnect_on_update_wrapper");
        }
        try {
            tracker->disconnectOnUpdate(handler_id);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack HandTracker disconnectOnUpdate failed: ") + e.what());
        } catch (const std::exception& e) { 
            throw;
        } catch (...) { 
            throw std::runtime_error("Unknown exception during Nuitrack HandTracker disconnectOnUpdate"); 
        }
    }

    uint64_t get_data_timestamp(const BridgedHandTrackerData& bht_data) {
        if (!bht_data.ptr) {
            throw std::runtime_error("Accessing null BridgedHandTrackerData in get_data_timestamp");
        }
        return bht_data.ptr->getTimestamp();
    }

    int32_t get_data_num_users(const BridgedHandTrackerData& bht_data) {
        if (!bht_data.ptr) {
            throw std::runtime_error("Accessing null BridgedHandTrackerData in get_data_num_users");
        }
        return static_cast<int32_t>(bht_data.ptr->getNumUsers());
    }

    size_t get_users_hands_vector_size(const BridgedHandTrackerData& bht_data) {
        if (!bht_data.ptr) {
            throw std::runtime_error("Accessing null BridgedHandTrackerData in get_users_hands_vector_size");
        }
        return bht_data.ptr->getUsersHands().size();
    }


    int32_t get_user_id_at(const BridgedHandTrackerData& bht_data, size_t user_vec_idx) {
        if (!bht_data.ptr) {
            throw std::runtime_error("Accessing null BridgedHandTrackerData in get_user_id_at");
        }
        // getUsersHands() returns a temporary vector. We operate on this copy.
        const auto users_hands_vector = bht_data.ptr->getUsersHands();
        if (user_vec_idx >= users_hands_vector.size()) {
            throw std::out_of_range("User index out of range in get_user_id_at");
        }
        // Copy the int value.
        return static_cast<int32_t>(users_hands_vector[user_vec_idx].userId);
    }

    std::shared_ptr<NuitrackHand> get_left_hand_at(const BridgedHandTrackerData& bht_data, size_t user_vec_idx) {
        if (!bht_data.ptr) {
            throw std::runtime_error("Accessing null BridgedHandTrackerData in get_left_hand_at");
        }
        const auto users_hands_vector = bht_data.ptr->getUsersHands(); // Temporary vector
        if (user_vec_idx >= users_hands_vector.size()) {
            throw std::out_of_range("User index out of range in get_left_hand_at");
        }
        // Copy the std::shared_ptr<NuitrackHand>. This increments the ref count and is safe.
        return users_hands_vector[user_vec_idx].leftHand;
    }

    std::shared_ptr<NuitrackHand> get_right_hand_at(const BridgedHandTrackerData& bht_data, size_t user_vec_idx) {
        if (!bht_data.ptr) {
            throw std::runtime_error("Accessing null BridgedHandTrackerData in get_right_hand_at");
        }
        const auto users_hands_vector = bht_data.ptr->getUsersHands(); // Temporary vector
        if (user_vec_idx >= users_hands_vector.size()) {
            throw std::out_of_range("User index out of range in get_right_hand_at");
        }
        // Copy the std::shared_ptr<NuitrackHand>.
        return users_hands_vector[user_vec_idx].rightHand;
    }

    float get_hand_x(const NuitrackHand& hand) { return hand.x; }
    float get_hand_y(const NuitrackHand& hand) { return hand.y; }
    bool get_hand_is_click(const NuitrackHand& hand) { return hand.click; }
    int32_t get_hand_pressure(const NuitrackHand& hand) { return static_cast<int32_t>(hand.pressure); }
    float get_hand_x_real(const NuitrackHand& hand) { return hand.xReal; }
    float get_hand_y_real(const NuitrackHand& hand) { return hand.yReal; }
    float get_hand_z_real(const NuitrackHand& hand) { return hand.zReal; }
} // namespace nuitrack_bridge::hand_tracker