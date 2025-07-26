#include "nuitrack_bridge/modules/gesture_recognizer.h"
#include "nuitrack/modules/GestureRecognizer.h"
#include "nuitrack/types/GestureData.h"
#include <stdexcept>
#include <string>

namespace nuitrack_bridge::gesture_recognizer {

    // --- Exception Formatting Helpers (copied from skeleton_tracker.cc) ---
    static std::string format_nuitrack_error(const std::string& function_name, const std::string& nuitrack_error_what) {
        return "Nuitrack " + function_name + " failed: " + nuitrack_error_what;
    }
    static std::string format_std_error(const std::string& function_name, const std::string& std_error_what) {
        return "Standard exception in Nuitrack " + function_name + ": " + std_error_what;
    }
    static std::string format_unknown_error(const std::string& function_name) {
        return "Unknown exception during Nuitrack " + function_name;
    }

    // --- Lifecycle ---
    std::shared_ptr<GestureRecognizer> createGestureRecognizer() {
        try {
            return tdv::nuitrack::GestureRecognizer::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("GestureRecognizer::create", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("GestureRecognizer::create", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("GestureRecognizer::create"));
        }
    }

    // --- Callback Implementations ---
    uint64_t connectOnCompletedGesturesFrameForAsync(const std::shared_ptr<GestureRecognizer>& recognizer, void* newGesturesSender) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in connectOnCompletedGesturesFrameForAsync");
        try {
            return recognizer->connectOnNewGestures([newGesturesSender](GestureData::Ptr frame) {
                rust_gesture_recognizer_completed_gestures_frame_dispatcher_async(frame, newGesturesSender);
            });
        } catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("connectOnNewGestures", e.what())); }
          catch (const std::exception& e) { throw std::runtime_error(format_std_error("connectOnNewGestures", e.what())); }
          catch (...) { throw std::runtime_error(format_unknown_error("connectOnNewGestures")); }
    }

    void disconnectOnCompletedGesturesFrame(const std::shared_ptr<GestureRecognizer>& recognizer, uint64_t handlerId) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in disconnectOnCompletedGesturesFrame");
        try { recognizer->disconnectOnNewGestures(handlerId); }
        catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("disconnectOnNewGestures", e.what())); }
        catch (...) { throw std::runtime_error(format_unknown_error("disconnectOnNewGestures")); }
    }

    uint64_t connectOnUserStateChangeForAsync(const std::shared_ptr<GestureRecognizer>& recognizer, void* userStateSender) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in connectOnUserStateChangeForAsync");
        try {
            return recognizer->connectOnUserStateChange([userStateSender](UserStateData::Ptr frame) {
                rust_gesture_recognizer_user_state_change_dispatcher_async(frame, userStateSender);
            });
        } catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("connectOnUserStateChange", e.what())); }
          catch (const std::exception& e) { throw std::runtime_error(format_std_error("connectOnUserStateChange", e.what())); }
          catch (...) { throw std::runtime_error(format_unknown_error("connectOnUserStateChange")); }
    }

    void disconnectOnUserStateChange(const std::shared_ptr<GestureRecognizer>& recognizer, uint64_t handlerId) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in disconnectOnUserStateChange");
        try { recognizer->disconnectOnUserStateChange(handlerId); }
        catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("disconnectOnUserStateChange", e.what())); }
        catch (...) { throw std::runtime_error(format_unknown_error("disconnectOnUserStateChange")); }
    }

    uint64_t connectOnUpdateForAsync(const std::shared_ptr<GestureRecognizer>& recognizer, void* userGesturesStateSender) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in connectOnUpdateForAsync");
        try {
            return recognizer->connectOnUpdate([userGesturesStateSender](UserGesturesStateData::Ptr frame) {
                rust_gesture_recognizer_update_dispatcher_async(frame, userGesturesStateSender);
            });
        } catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("connectOnUpdate", e.what())); }
          catch (const std::exception& e) { throw std::runtime_error(format_std_error("connectOnUpdate", e.what())); }
          catch (...) { throw std::runtime_error(format_unknown_error("connectOnUpdate")); }
    }

    void disconnectOnUpdate(const std::shared_ptr<GestureRecognizer>& recognizer, uint64_t handlerId) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in disconnectOnUpdate");
        try { recognizer->disconnectOnUpdate(handlerId); }
        catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("disconnectOnUpdate", e.what())); }
        catch (...) { throw std::runtime_error(format_unknown_error("disconnectOnUpdate")); }
    }

    // --- Configuration & Control ---
    void setControlGesturesStatus(const std::shared_ptr<GestureRecognizer>& recognizer, bool status) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in setControlGesturesStatus");
        try { recognizer->setControlGesturesStatus(status); }
        catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("setControlGesturesStatus", e.what())); }
        catch (...) { throw std::runtime_error(format_unknown_error("setControlGesturesStatus")); }
    }

    // --- Module Information ---
    float getProcessingTime(const std::shared_ptr<GestureRecognizer>& recognizer) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in getProcessingTime");
        try { return recognizer->getProcessingTime(); }
        catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("getProcessingTime", e.what())); }
        catch (...) { throw std::runtime_error(format_unknown_error("getProcessingTime")); }
    }

    uint64_t getRecognizerTimestamp(const std::shared_ptr<GestureRecognizer>& recognizer) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in getRecognizerTimestamp");
        try { return recognizer->getTimestamp(); }
        catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("getTimestamp", e.what())); }
        catch (...) { throw std::runtime_error(format_unknown_error("getTimestamp")); }
    }
    
    bool canUpdate(const std::shared_ptr<GestureRecognizer>& recognizer) {
        if (!recognizer) throw std::runtime_error("GestureRecognizer is null in canUpdate");
        try { return recognizer->canUpdate(); }
        catch (const tdv::nuitrack::Exception& e) { throw std::runtime_error(format_nuitrack_error("canUpdate", e.what())); }
        catch (...) { throw std::runtime_error(format_unknown_error("canUpdate")); }
    }
}