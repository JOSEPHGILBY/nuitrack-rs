#include "nuitrack_bridge/modules/hand_tracker.h"
#include "nuitrack/modules/HandTracker.h"
#include "nuitrack/types/HandTrackerData.h"
#include "nuitrack/Nuitrack.h" // For tdv::nuitrack::Exception

#include <stdexcept> // For std::runtime_error
#include <string>    // For std::string
#include <functional> // For std::function (used by Nuitrack callbacks)

namespace nuitrack_bridge::hand_tracker {

    // Reusing error formatting helpers (can be in a shared utility header if used by many modules)
    static std::string format_nuitrack_error(const std::string& function_name, const std::string& nuitrack_error_what) {
        return "Nuitrack " + function_name + " failed: " + nuitrack_error_what;
    }

    static std::string format_std_error(const std::string& function_name, const std::string& std_error_what) {
        return "Standard exception in Nuitrack " + function_name + ": " + std_error_what;
    }

    static std::string format_unknown_error(const std::string& function_name) {
        return "Unknown exception during Nuitrack " + function_name;
    }

    std::shared_ptr<HandTracker> createHandTracker() {
        try {
            return tdv::nuitrack::HandTracker::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("HandTracker::create", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("HandTracker::create", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("HandTracker::create"));
        }
    }

    uint64_t connectOnUpdateForAsync(
        const std::shared_ptr<HandTracker>& tracker,
        void* handFrameSender
    ) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in connectOnUpdateForAsync");
        }
        try {
            return tracker->connectOnUpdate(
                [handFrameSender](tdv::nuitrack::HandTrackerData::Ptr data) {
                    // Call the Rust dispatcher
                    rust_hand_tracker_hand_frame_dispatcher_async(data, handFrameSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("HandTracker::connectOnUpdate", e.what()));
        } catch (const std::exception& e) { // Catches rust::Error too
            throw std::runtime_error(format_std_error("HandTracker::connectOnUpdate", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("HandTracker::connectOnUpdate"));
        }
    }

    void disconnectOnUpdate(
        const std::shared_ptr<HandTracker>& tracker,
        uint64_t handlerId
    ) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in disconnectOnUpdate");
        }
        try {
            tracker->disconnectOnUpdate(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("HandTracker::disconnectOnUpdate", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("HandTracker::disconnectOnUpdate", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("HandTracker::disconnectOnUpdate"));
        }
    }

    std::shared_ptr<HandData> getData(const std::shared_ptr<HandTracker>& tracker) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in getData");
        }
        try {
            return tracker->getData();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("HandTracker::getData", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("HandTracker::getData", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("HandTracker::getData"));
        }
    }

    float getProcessingTime(const std::shared_ptr<HandTracker>& tracker) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in getProcessingTime");
        }
        try {
            return tracker->getProcessingTime();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("HandTracker::getProcessingTime", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("HandTracker::getProcessingTime", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("HandTracker::getProcessingTime"));
        }
    }

    bool canUpdate(const std::shared_ptr<HandTracker>& tracker) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in canUpdate");
        }
        try {
            return tracker->canUpdate(); // Inherited from tdv::nuitrack::Module
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("HandTracker::canUpdate (Module)", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("HandTracker::canUpdate (Module)", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("HandTracker::canUpdate (Module)"));
        }
    }

    uint64_t getTrackerTimestamp(const std::shared_ptr<HandTracker>& tracker) {
        if (!tracker) {
            throw std::runtime_error("HandTracker instance is null in getTrackerTimestamp");
        }
        try {
            return tracker->getTimestamp(); // Inherited from tdv::nuitrack::Module
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("HandTracker::getTimestamp (Module)", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("HandTracker::getTimestamp (Module)", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("HandTracker::getTimestamp (Module)"));
        }
    }

}