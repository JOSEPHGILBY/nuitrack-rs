#pragma once

#include "rust/cxx.h"
#include <memory>

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class HandTracker;
    class HandTrackerData;
    using HandData = HandTrackerData;
}

namespace nuitrack_bridge::hand_tracker {

    // Using aliases for Nuitrack types
    using HandTracker = tdv::nuitrack::HandTracker;
    using HandData = tdv::nuitrack::HandData;

    using c_void = void; // Alias for Rust's *mut c_void

    // --- Module Creation ---
    std::shared_ptr<HandTracker> createHandTracker();

    // --- Callback Management ---
    uint64_t connectOnUpdateForAsync(
        const std::shared_ptr<HandTracker>& tracker,
        void* handFrameSender // Generic void pointer for Rust's sender
    );

    void disconnectOnUpdate(
        const std::shared_ptr<HandTracker>& tracker,
        uint64_t handlerId
    );

    // --- Synchronous Data Access ---
    std::shared_ptr<HandData> getData(const std::shared_ptr<HandTracker>& tracker);

    // --- Module Information ---
    float getProcessingTime(const std::shared_ptr<HandTracker>& tracker);
    bool canUpdate(const std::shared_ptr<HandTracker>& tracker);
    uint64_t getTrackerTimestamp(const std::shared_ptr<HandTracker>& tracker); // To distinguish from HandTrackerData's timestamp

} // namespace nuitrack_bridge::hand_tracker

extern "C" {
    // Rust dispatcher function to be called by the C++ lambda
    void rust_hand_tracker_on_update_dispatcher(
        std::shared_ptr<tdv::nuitrack::HandTrackerData>& data,
        void* handFrameSender
    );
}
