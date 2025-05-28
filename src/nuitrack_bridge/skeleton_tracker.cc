#include "nuitrack_bridge/skeleton_tracker.h"
#include "nuitrack/modules/SkeletonTracker.h"

#include "nuitrack/Nuitrack.h"
#include <stdexcept>
#include <string>

namespace nuitrack_bridge::skeleton_tracker {
    std::shared_ptr<SkeletonTracker> create_skeleton_tracker() {
        try {
            return tdv::nuitrack::SkeletonTracker::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack SkeletonTracker::create failed: ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error(std::string("Exception during Nuitrack SkeletonTracker::create: ") + e.what());
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack SkeletonTracker::create");
        }
    }
}