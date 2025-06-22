#include "nuitrack_bridge/modules/skeleton_tracker.h"
#include "nuitrack/modules/SkeletonTracker.h"
#include "nuitrack/types/SkeletonData.h"
#include "nuitrack/Nuitrack.h"

#include <stdexcept>
#include <string>
#include <functional> 

namespace nuitrack_bridge::skeleton_tracker {

    static std::string format_nuitrack_error(const std::string& function_name, const std::string& nuitrack_error_what) {
        return "Nuitrack " + function_name + " failed: " + nuitrack_error_what;
    }

    static std::string format_std_error(const std::string& function_name, const std::string& std_error_what) {
        return "Standard exception in Nuitrack " + function_name + ": " + std_error_what;
    }

    static std::string format_unknown_error(const std::string& function_name) {
        return "Unknown exception during Nuitrack " + function_name;
    }

    std::shared_ptr<SkeletonTracker> createSkeletonTracker() {
        try {
            return tdv::nuitrack::SkeletonTracker::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::create", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::create", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::create"));
        }
    }

    uint64_t connectOnUpdateForAsync(
        const std::shared_ptr<SkeletonTracker>& tracker,
        void* skeletonFrameSender
    ) {
        if (!tracker) {
            throw std::runtime_error("SkeletonTracker instance is null in connectOnUpdateForAsync");
        }
        try {
            return tracker->connectOnUpdate(
                [skeletonFrameSender](SkeletonData::Ptr frame) {
                    // Exceptions from the Rust callback are handled by CXX,
                    // typically by converting Rust panics into C++ exceptions (rust::Error).
                    // These would be caught by the catch blocks below if they propagate.
                    rust_skeleton_tracker_skeleton_frame_dispatcher_async(frame, skeletonFrameSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::connectOnUpdate", e.what()));
        } catch (const std::exception& e) { // Catches rust::Error too, as it derives from std::exception
            throw std::runtime_error(format_std_error("SkeletonTracker::connectOnUpdate", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::connectOnUpdate"));
        }
    }

    void disconnectOnUpdate(
        const std::shared_ptr<SkeletonTracker>& tracker,
        uint64_t handlerId
    ) {
        if (!tracker) {
            throw std::runtime_error("SkeletonTracker instance is null in disconnectOnUpdate");
        }
        try {
            tracker->disconnectOnUpdate(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::disconnectOnUpdate", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::disconnectOnUpdate", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::disconnectOnUpdate"));
        }
    }

    uint64_t connectOnNewUserForAsync(
        const std::shared_ptr<SkeletonTracker>& tracker,
        void* newUserFrameSender
    ) {
        if (!tracker) {
            throw std::runtime_error("SkeletonTracker instance is null in connectOnNewUserForAsync");
        }
        try {
            return tracker->connectOnNewUser(
                [newUserFrameSender](tdv::nuitrack::SkeletonTracker::Ptr /* st_ptr */, int userID) {
                    rust_skeleton_tracker_new_user_event_dispatcher_async(userID, newUserFrameSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::connectOnNewUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::connectOnNewUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::connectOnNewUser"));
        }
    }

    void disconnectOnNewUser(
        const std::shared_ptr<SkeletonTracker>& tracker,
        uint64_t handlerId
    ) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in disconnectOnNewUser"); }
        try {
            tracker->disconnectOnNewUser(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::disconnectOnNewUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::disconnectOnNewUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::disconnectOnNewUser"));
        }
    }

    uint64_t connectOnLostUserForAsync(
        const std::shared_ptr<SkeletonTracker>& tracker,
        void* lostUserFrameSender
    ) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in connectOnLostUserForAsync"); }
        try {
            return tracker->connectOnLostUser(
                [lostUserFrameSender](tdv::nuitrack::SkeletonTracker::Ptr /* st_ptr */, int userID) {
                    rust_skeleton_tracker_lost_user_event_dispatcher_async(userID, lostUserFrameSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::connectOnLostUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::connectOnLostUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::connectOnLostUser"));
        }
    }

    void disconnectOnLostUser(
        const std::shared_ptr<SkeletonTracker>& tracker,
        uint64_t handlerId
    ) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in disconnectOnLostUser"); }
        try {
            tracker->disconnectOnLostUser(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::disconnectOnLostUser", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::disconnectOnLostUser", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::disconnectOnLostUser"));
        }
    }

    void setNumActiveUsers(const std::shared_ptr<SkeletonTracker>& tracker, int numUsers) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in setNumActiveUsers"); }
        try { 
            tracker->setNumActiveUsers(numUsers); 
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::setNumActiveUsers", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::setNumActiveUsers", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::setNumActiveUsers"));
        }
    }

    bool isAutoTracking(const std::shared_ptr<SkeletonTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in isAutoTracking"); }
        try { 
            return tracker->isAutoTracking(); 
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::isAutoTracking", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::isAutoTracking", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::isAutoTracking"));
        }

        return false; // ErrorState: This should be unreachable
    }

    void setAutoTracking(const std::shared_ptr<SkeletonTracker>& tracker, bool tracking) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in setAutoTracking"); }
        try { 
            tracker->setAutoTracking(tracking); 
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::setAutoTracking", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::setAutoTracking", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::setAutoTracking"));
        }
    }

    void startTracking(const std::shared_ptr<SkeletonTracker>& tracker, int userID) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in startTracking"); }
        try { 
            tracker->startTracking(userID); 
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::startTracking", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::startTracking", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::startTracking"));
        }
    }

    void stopTracking(const std::shared_ptr<SkeletonTracker>& tracker, int userID) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in stopTracking"); }
        try { 
            tracker->stopTracking(userID); 
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::stopTracking", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::stopTracking", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::stopTracking"));
        }
    }

    bool isTracking(const std::shared_ptr<SkeletonTracker>& tracker, int userID) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in isTracking"); }
        try { 
            return tracker->isTracking(userID); 
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::isTracking", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::isTracking", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::isTracking"));
        }
    }

    std::shared_ptr<SkeletonData> getSkeletons(const std::shared_ptr<SkeletonTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in getSkeletons"); }
        try {
            return tracker->getSkeletons();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::getSkeletons", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::getSkeletons", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::getSkeletons"));
        }
    }

    float getProcessingTime(const std::shared_ptr<SkeletonTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in getProcessingTime"); }
        try { 
            return tracker->getProcessingTime(); 
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::getProcessingTime", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::getProcessingTime", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::getProcessingTime"));
        }
    }

    uint64_t getTrackerTimestamp(const std::shared_ptr<SkeletonTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in getTrackerTimestamp"); }
        try { 
            return tracker->getTimestamp(); // This is tdv::nuitrack::Module::getTimestamp
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::getTimestamp (Module)", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::getTimestamp (Module)", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::getTimestamp (Module)"));
        }
    }

    bool canUpdate(const std::shared_ptr<SkeletonTracker>& tracker) {
        if (!tracker) { throw std::runtime_error("SkeletonTracker instance is null in canUpdate"); }
        try { 
            return tracker->canUpdate(); // This is tdv::nuitrack::Module::canUpdate
        } catch (const tdv::nuitrack::Exception& e) { 
            throw std::runtime_error(format_nuitrack_error("SkeletonTracker::canUpdate (Module)", e.what())); 
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("SkeletonTracker::canUpdate (Module)", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("SkeletonTracker::canUpdate (Module)"));
        }
    }

    
}