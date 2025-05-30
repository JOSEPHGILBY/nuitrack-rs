#include "nuitrack/Nuitrack.h"
#include "nuitrack_bridge/core.h"
#include <stdexcept>
#include <string>

namespace nuitrack_bridge::core {
    
    void init(rust::Str configPath) {
        std::string configPathCvt(configPath);
        try {
            Nuitrack::init(configPathCvt);
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
            Nuitrack::run();
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
            Nuitrack::update();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack update failed: ") + e.what());
        } catch (const std::exception& e) {
            throw;
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack update");
        }
    }

    void waitUpdateHandTracker(const std::shared_ptr<HandTracker>& handTracker) {
        try {
            Nuitrack::waitUpdate(handTracker);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { // Handle specific exceptions if needed
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate: ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate failed: ") + e.what());
        } // ...
    }

    void waitUpdateSkeletonTracker(const std::shared_ptr<SkeletonTracker>& skeletonTracker) {
        try {
            Nuitrack::waitUpdate(skeletonTracker);
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
            Nuitrack::release();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack release failed: ") + e.what());
        } catch (const std::exception& e) {
            throw;
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack release");
        }
    }
}