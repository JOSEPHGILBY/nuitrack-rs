#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector> // For std::vector

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class SkeletonTracker;
    class SkeletonData;
}

namespace nuitrack_bridge::skeleton_tracker {

    using SkeletonTracker = tdv::nuitrack::SkeletonTracker;
    using SkeletonData = tdv::nuitrack::SkeletonData;

    using c_void = void;

    std::shared_ptr<SkeletonTracker> createSkeletonTracker();

    uint64_t connectOnUpdateForAsync(
        const std::shared_ptr<SkeletonTracker>& tracker,
        void* skeletonFrameSender
    );

    void disconnectOnUpdate(
        const std::shared_ptr<SkeletonTracker>& tracker,
        uint64_t handlerId
    );

     // --- New User Callbacks ---
    uint64_t connectOnNewUserForAsync(
        const std::shared_ptr<SkeletonTracker>& tracker,
        void* newUserFrameSender
    );

    void disconnectOnNewUser(
        const std::shared_ptr<SkeletonTracker>& tracker,
        uint64_t handlerId
    );

    // --- Lost User Callbacks ---
    uint64_t connectOnLostUserForAsync(
        const std::shared_ptr<SkeletonTracker>& tracker,
        void* lostUserFrameSender
    );

    void disconnectOnLostUser(
        const std::shared_ptr<SkeletonTracker>& tracker,
        uint64_t handlerId
    );

    // --- Configuration & Control ---
    void setNumActiveUsers(
        const std::shared_ptr<SkeletonTracker>& tracker,
        int numUsers
    );

    bool isAutoTracking(const std::shared_ptr<SkeletonTracker>& tracker);

    void setAutoTracking(
        const std::shared_ptr<SkeletonTracker>& tracker,
        bool tracking
    );

    void startTracking(
        const std::shared_ptr<SkeletonTracker>& tracker,
        int userID
    );

    void stopTracking(
        const std::shared_ptr<SkeletonTracker>& tracker,
        int userID
    );

    bool isTracking(
        const std::shared_ptr<SkeletonTracker>& tracker,
        int userID
    );

    // --- Synchronous Data Access ---
    std::shared_ptr<SkeletonData> getSkeletons(const std::shared_ptr<SkeletonTracker>& tracker);

    // --- Module Information ---
    float getProcessingTime(const std::shared_ptr<SkeletonTracker>& tracker);

    // Renamed to avoid potential conflicts if SkeletonData also has getTimestamp in FFI
    uint64_t getTrackerTimestamp(const std::shared_ptr<SkeletonTracker>& tracker); 

    bool canUpdate(const std::shared_ptr<SkeletonTracker>& tracker);
    

}


extern "C" {
    void rust_skeleton_tracker_skeleton_frame_dispatcher_async(
        std::shared_ptr<tdv::nuitrack::SkeletonData>& data,
        void* skeletonFrameSender
    );

    // New dispatcher declarations for C++ to call Rust
    void rust_skeleton_tracker_new_user_event_dispatcher_async(
        int userID,
        void* newUserFrameSender
    );

    void rust_skeleton_tracker_lost_user_event_dispatcher_async(
        int userID,
        void* lostUserFrameSender
    );
}
