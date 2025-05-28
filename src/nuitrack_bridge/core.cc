#include "nuitrack/Nuitrack.h"
#include "nuitrack_bridge/core.h"
#include <stdexcept>
#include <string>

namespace nuitrack_bridge::core {
    
    void init(rust::Str config_path_str) {
        std::string config_path(config_path_str);
        try {
            tdv::nuitrack::Nuitrack::init(config_path);
        }
        catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack init failed (tdv::nuitrack::Exception): ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error("Unknown exception during Nuitrack initialization");
        }
    }

    // New implementation for run wrapper
    void run() {
        try {
            // The static Nuitrack::run() method itself handles C API calls and exception translation
            tdv::nuitrack::Nuitrack::run();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack run failed: ") + e.what());
        } catch (const std::exception& e) {
            throw;
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack run");
        }
    }

    void update() {
        try {
            tdv::nuitrack::Nuitrack::update();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack update failed: ") + e.what());
        } catch (const std::exception& e) {
            throw;
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack update");
        }
    }

    void waitUpdateHandTracker(const std::shared_ptr<tdv::nuitrack::HandTracker>& hand_tracker_ptr) {
        try {
            tdv::nuitrack::Nuitrack::waitUpdate(hand_tracker_ptr);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { // Handle specific exceptions if needed
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate: ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate failed: ") + e.what());
        } // ...
    }

    void waitUpdateSkeletonTracker(const std::shared_ptr<tdv::nuitrack::SkeletonTracker>& skeleton_tracker_ptr) {
        try {
            tdv::nuitrack::Nuitrack::waitUpdate(skeleton_tracker_ptr);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { 
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate(SkeletonTracker): ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate(SkeletonTracker) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error(std::string("Exception during Nuitrack waitUpdate(SkeletonTracker): ") + e.what());
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack waitUpdate(SkeletonTracker)");
        }
    }

    // New implementation for release wrapper
    void release() {
        try {
            // The static Nuitrack::release() method handles C API calls,
            // internal callback struct cleanup, and exception translation
            tdv::nuitrack::Nuitrack::release();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack release failed: ") + e.what());
        } catch (const std::exception& e) {
            throw;
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack release");
        }
    }
}