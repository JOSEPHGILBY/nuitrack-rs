#pragma once

#include "rust/cxx.h"
#include <memory>
#include <vector>

// Forward declare Nuitrack types
namespace tdv::nuitrack {
    class GestureRecognizer;
    class GestureData;
    class UserStateData;
    class UserGesturesStateData;
}

namespace nuitrack_bridge::gesture_recognizer {

    // Aliases for convenience
    using GestureRecognizer = tdv::nuitrack::GestureRecognizer;
    using GestureData = tdv::nuitrack::GestureData;
    using UserStateData = tdv::nuitrack::UserStateData;
    using UserGesturesStateData = tdv::nuitrack::UserGesturesStateData;
    using c_void = void;

    // --- Lifecycle ---
    std::shared_ptr<GestureRecognizer> createGestureRecognizer();

    // --- Callbacks ---
    uint64_t connectOnCompletedGesturesFrameForAsync(
        const std::shared_ptr<GestureRecognizer>& recognizer,
        void* newGesturesSender
    );

    void disconnectOnCompletedGesturesFrame(
        const std::shared_ptr<GestureRecognizer>& recognizer,
        uint64_t handlerId
    );

    uint64_t connectOnUserStateChangeForAsync(
        const std::shared_ptr<GestureRecognizer>& recognizer,
        void* userStateSender
    );

    void disconnectOnUserStateChange(
        const std::shared_ptr<GestureRecognizer>& recognizer,
        uint64_t handlerId
    );

    uint64_t connectOnUpdateForAsync(
        const std::shared_ptr<GestureRecognizer>& recognizer,
        void* userGesturesStateSender
    );

    void disconnectOnUpdate(
        const std::shared_ptr<GestureRecognizer>& recognizer,
        uint64_t handlerId
    );

    // --- Configuration & Control ---
    void setControlGesturesStatus(
        const std::shared_ptr<GestureRecognizer>& recognizer,
        bool status
    );

    // --- Module Information ---
    float getProcessingTime(const std::shared_ptr<GestureRecognizer>& recognizer);
    uint64_t getRecognizerTimestamp(const std::shared_ptr<GestureRecognizer>& recognizer);
    bool canUpdate(const std::shared_ptr<GestureRecognizer>& recognizer);
}

// Rust functions that C++ will call from the async callbacks
extern "C" {
    void rust_gesture_recognizer_completed_gestures_frame_dispatcher_async(
        std::shared_ptr<tdv::nuitrack::GestureData>& data,
        void* newGesturesSender
    );

    void rust_gesture_recognizer_user_state_change_dispatcher_async(
        std::shared_ptr<tdv::nuitrack::UserStateData>& data,
        void* userStateSender
    );

    void rust_gesture_recognizer_update_dispatcher_async(
        std::shared_ptr<tdv::nuitrack::UserGesturesStateData>& data,
        void* userGesturesStateSender
    );
}