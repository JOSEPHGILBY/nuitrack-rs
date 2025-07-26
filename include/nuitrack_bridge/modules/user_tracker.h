#pragma once

#include "rust/cxx.h"
#include <memory>

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class UserTracker;
    class UserFrame;
}

namespace nuitrack_bridge::user_tracker {

    using UserTracker = tdv::nuitrack::UserTracker;
    using UserFrame = tdv::nuitrack::UserFrame;

    using c_void = void;

    // --- Module Lifecycle ---
    std::shared_ptr<UserTracker> createUserTracker();

    // --- On-Update Callbacks ---
    uint64_t connectOnUpdateForAsync(
        const std::shared_ptr<UserTracker>& tracker,
        void* userFrameSender
    );

    void disconnectOnUpdate(
        const std::shared_ptr<UserTracker>& tracker,
        uint64_t handlerId
    );

    // --- New User Callbacks ---
    uint64_t connectOnNewUserForAsync(
        const std::shared_ptr<UserTracker>& tracker,
        void* newUserEventSender
    );

    void disconnectOnNewUser(
        const std::shared_ptr<UserTracker>& tracker,
        uint64_t handlerId
    );

    // --- Lost User Callbacks ---
    uint64_t connectOnLostUserForAsync(
        const std::shared_ptr<UserTracker>& tracker,
        void* lostUserEventSender
    );

    void disconnectOnLostUser(
        const std::shared_ptr<UserTracker>& tracker,
        uint64_t handlerId
    );

    // --- Synchronous Data Access ---
    std::shared_ptr<UserFrame> getUserFrame(const std::shared_ptr<UserTracker>& tracker);

    // --- Module Information ---
    float getProcessingTime(const std::shared_ptr<UserTracker>& tracker);
    uint64_t getTrackerTimestamp(const std::shared_ptr<UserTracker>& tracker);
    bool canUpdate(const std::shared_ptr<UserTracker>& tracker);
}

// Dispatcher functions to be implemented in Rust and called by C++ lambdas
extern "C" {
    void rust_user_tracker_user_frame_dispatcher_async(
        std::shared_ptr<tdv::nuitrack::UserFrame>& data,
        void* userFrameSender
    );

    void rust_user_tracker_new_user_event_dispatcher_async(
        int userID,
        void* newUserEventSender
    );

    void rust_user_tracker_lost_user_event_dispatcher_async(
        int userID,
        void* lostUserEventSender
    );
}