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

    void waitUpdateColorSensor(const std::shared_ptr<ColorSensor>& colorSensor) {
        try {
            Nuitrack::waitUpdate(colorSensor);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { // Handle specific exceptions if needed
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate(ColorSensor): ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate(ColorSensor) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error(std::string("Exception during Nuitrack waitUpdate(ColorSensor): ") + e.what());
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack waitUpdate(ColorSensor)");
        }
    }

    void waitUpdateHandTracker(const std::shared_ptr<HandTracker>& handTracker) {
        try {
            Nuitrack::waitUpdate(handTracker);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { // Handle specific exceptions if needed
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate(HandTracker): ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate(HandTracker) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error(std::string("Exception during Nuitrack waitUpdate(HandTracker): ") + e.what());
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack waitUpdate(HandTracker)");
        }
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

    void waitUpdateDepthSensor(const std::shared_ptr<DepthSensor>& depthSensor) {
        try {
            Nuitrack::waitUpdate(depthSensor);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { 
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate(DepthSensor): ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate(DepthSensor) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error(std::string("Exception during Nuitrack waitUpdate(DepthSensor): ") + e.what());
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack waitUpdate(DepthSensor)");
        }
    }

    void waitUpdateUserTracker(const std::shared_ptr<UserTracker>& userTracker) {
        try {
            Nuitrack::waitUpdate(userTracker);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { 
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate(UserTracker): ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate(UserTracker) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error(std::string("Exception during Nuitrack waitUpdate(UserTracker): ") + e.what());
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack waitUpdate(UserTracker)");
        }
    }

    void waitUpdateGestureRecognizer(const std::shared_ptr<GestureRecognizer>& gestureRecognizer) {
        try {
            Nuitrack::waitUpdate(gestureRecognizer);
        } catch (const tdv::nuitrack::LicenseNotAcquiredException& e) { 
            throw std::runtime_error(std::string("LicenseNotAcquiredException during waitUpdate(GestureRecognizer): ") + e.what());
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack waitUpdate(GestureRecognizer) failed: ") + e.what());
        } catch (const std::exception& e) {
            throw std::runtime_error(std::string("Exception during Nuitrack waitUpdate(GestureRecognizer): ") + e.what());
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack waitUpdate(GestureRecognizer)");
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

    void setConfigValue(rust::Str key, rust::Str value) {
        try {
            Nuitrack::setConfigValue(std::string(key), std::string(value));
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack setConfigValue failed: ") + e.what());
        }
    }

    rust::String getConfigValue(rust::Str key) {
        try {
            return Nuitrack::getConfigValue(std::string(key));
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(std::string("Nuitrack getConfigValue failed: ") + e.what());
        }
    }
}