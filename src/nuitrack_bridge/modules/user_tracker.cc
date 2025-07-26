#include "nuitrack_bridge/modules/user_tracker.h"
#include "nuitrack/modules/UserTracker.h"
#include "nuitrack/types/UserFrame.h"
#include "nuitrack/Nuitrack.h"

#include <stdexcept>
#include <string>
#include <functional>

namespace nuitrack_bridge::user_tracker {

    // --- Helper Functions for Error Formatting ---
    static std::string format_nuitrack_error(const std::string& function_name, const std::string& nuitrack_error_what) {
        return "Nuitrack " + function_name + " failed: " + nuitrack_error_what;
    }
    static std::string format_std_error(const std::string& function_name, const std::string& std_error_what) {
        return "Standard exception in Nuitrack " + function_name + ": " + std_error_what;
    }
    static std::string format_unknown_error(const std::string& function_name) {
        return "Unknown exception during Nuitrack " + function_name;
    }

    // --- Bridge Function Implementations ---
    std::shared_ptr<UserTracker> createUserTracker() {
        try {
            return tdv::nuitrack::UserTracker::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::create", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::create", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::create"));
        }
    }

    uint64_t connectOnUpdateForAsync(const std::shared_ptr<UserTracker>& tracker, void* userFrameSender) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in connectOnUpdateForAsync"); }
        try {
            return tracker->connectOnUpdate(
                [userFrameSender](UserFrame::Ptr frame) {
                    rust_user_tracker_user_frame_dispatcher_async(frame, userFrameSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::connectOnUpdate", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::connectOnUpdate", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::connectOnUpdate"));
        }
    }

    void disconnectOnUpdate(const std::shared_ptr<UserTracker>& tracker, uint64_t handlerId) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in disconnectOnUpdate"); }
        try {
            tracker->disconnectOnUpdate(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::disconnectOnUpdate", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::disconnectOnUpdate", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::disconnectOnUpdate"));
        }
    }

    uint64_t connectOnNewUserForAsync(const std::shared_ptr<UserTracker>& tracker, void* newUserEventSender) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in connectOnNewUserForAsync"); }
        try {
            return tracker->connectOnNewUser(
                [newUserEventSender](int userID) {
                    rust_user_tracker_new_user_event_dispatcher_async(userID, newUserEventSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::connectOnNewUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::connectOnNewUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::connectOnNewUser"));
        }
    }

    void disconnectOnNewUser(const std::shared_ptr<UserTracker>& tracker, uint64_t handlerId) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in disconnectOnNewUser"); }
        try {
            tracker->disconnectOnNewUser(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::disconnectOnNewUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::disconnectOnNewUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::disconnectOnNewUser"));
        }
    }

    uint64_t connectOnLostUserForAsync(const std::shared_ptr<UserTracker>& tracker, void* lostUserEventSender) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in connectOnLostUserForAsync"); }
        try {
            return tracker->connectOnLostUser(
                [lostUserEventSender](int userID) {
                    rust_user_tracker_lost_user_event_dispatcher_async(userID, lostUserEventSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::connectOnLostUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::connectOnLostUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::connectOnLostUser"));
        }
    }

    void disconnectOnLostUser(const std::shared_ptr<UserTracker>& tracker, uint64_t handlerId) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in disconnectOnLostUser"); }
        try {
            tracker->disconnectOnLostUser(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::disconnectOnLostUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::disconnectOnLostUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::disconnectOnLostUser"));
        }
    }

    std::shared_ptr<UserFrame> getUserFrame(const std::shared_ptr<UserTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in getUserFrame"); }
        try {
            return tracker->getUserFrame();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::getUserFrame", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::getUserFrame", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::getUserFrame"));
        }
    }
    
    float getProcessingTime(const std::shared_ptr<UserTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in getProcessingTime"); }
        try {
            return tracker->getProcessingTime();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::getProcessingTime", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::getProcessingTime", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::getProcessingTime"));
        }
    }

    uint64_t getTrackerTimestamp(const std::shared_ptr<UserTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in getTrackerTimestamp"); }
        try {
            return tracker->getTimestamp();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::getTimestamp", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::getTimestamp", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::getTimestamp"));
        }
    }
    
    bool canUpdate(const std::shared_ptr<UserTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("UserTracker instance is null in canUpdate"); }
        try {
            return tracker->canUpdate();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("UserTracker::canUpdate", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("UserTracker::canUpdate", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("UserTracker::canUpdate"));
        }
    }

}