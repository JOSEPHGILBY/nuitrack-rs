#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector> // For std::vector

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class HandTracker;
    class HandTrackerData;
    struct UserHands; 
    struct Hand;
}

namespace nuitrack_bridge::hand_tracker {

    // Aliases for Nuitrack types
    using HandTracker = tdv::nuitrack::HandTracker;
    // using HandTrackerData = tdv::nuitrack::HandTrackerData;
    using NuitrackHand = tdv::nuitrack::Hand; // Alias for tdv::nuitrack::Hand to avoid ambiguity

    using c_void = void; // <--- ADD THIS ALIAS

    struct BridgedHandTrackerData {
        std::shared_ptr<tdv::nuitrack::HandTrackerData> ptr;

        // Constructor to make it easy to create
        BridgedHandTrackerData(std::shared_ptr<tdv::nuitrack::HandTrackerData> p) : ptr(std::move(p)) {}
    };

    // --- HandTracker Methods ---
    std::shared_ptr<HandTracker> create_hand_tracker();

    uint64_t connect_on_update_wrapper(
        const std::shared_ptr<HandTracker>& tracker,
        rust::Fn<void(std::shared_ptr<BridgedHandTrackerData>)> callback);

    uint64_t connect_on_update_with_user_data(
        const std::shared_ptr<HandTracker>& tracker,
        void* user_data // This will be the Rust MPSC sender passed from Rust
    );
    
    uint64_t connect_on_update_for_blocking(
        const std::shared_ptr<HandTracker>& tracker,
        void* user_data
    );

    void disconnect_on_update_wrapper(
        const std::shared_ptr<HandTracker>& tracker,
        uint64_t handler_id);


    uint64_t get_data_timestamp(const BridgedHandTrackerData& bht_data);
    int32_t get_data_num_users(const BridgedHandTrackerData& bht_data);
    size_t get_users_hands_vector_size(const BridgedHandTrackerData& bht_data);

    int32_t get_user_id_at(const BridgedHandTrackerData& bht_data, size_t user_vec_idx);
    std::shared_ptr<NuitrackHand> get_left_hand_at(const BridgedHandTrackerData& bht_data, size_t user_vec_idx);
    std::shared_ptr<NuitrackHand> get_right_hand_at(const BridgedHandTrackerData& bht_data, size_t user_vec_idx);

    float get_hand_x(const NuitrackHand& hand);
    float get_hand_y(const NuitrackHand& hand);
    bool get_hand_is_click(const NuitrackHand& hand); // Renamed for clarity
    int32_t get_hand_pressure(const NuitrackHand& hand);
    float get_hand_x_real(const NuitrackHand& hand);
    float get_hand_y_real(const NuitrackHand& hand);
    float get_hand_z_real(const NuitrackHand& hand);

}

extern "C" {
    void rust_hand_tracker_callback_dispatcher(
        std::shared_ptr<nuitrack_bridge::hand_tracker::BridgedHandTrackerData> data,
        void* user_data
    );

    void rust_blocking_hand_tracker_trampoline(
        std::shared_ptr<nuitrack_bridge::hand_tracker::BridgedHandTrackerData> data,
        void* user_data
    );
}