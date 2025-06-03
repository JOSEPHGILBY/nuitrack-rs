#include "nuitrack_bridge/modules/color_sensor.h"
#include "nuitrack/modules/ColorSensor.h"
#include "nuitrack/types/RGBFrame.h"
#include "nuitrack/types/OutputMode.h" // For tdv::nuitrack::OutputMode
#include "nuitrack/Nuitrack.h"      // For tdv::nuitrack::Exception

#include <stdexcept>
#include <string>
#include <functional>

// Helper error formatting functions (replicated for this module)
namespace {
    std::string format_nuitrack_error(const std::string& function_name, const std::string& nuitrack_error_what) {
        return "Nuitrack " + function_name + " failed: " + nuitrack_error_what;
    }

    std::string format_std_error(const std::string& function_name, const std::string& std_error_what) {
        return "Standard exception in Nuitrack " + function_name + ": " + std_error_what;
    }

    std::string format_unknown_error(const std::string& function_name) {
        return "Unknown exception during Nuitrack " + function_name;
    }
} // anonymous namespace

namespace nuitrack_bridge::color_sensor {

    std::shared_ptr<ColorSensor> createColorSensor() {
        try {
            return tdv::nuitrack::ColorSensor::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("ColorSensor::create", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("ColorSensor::create", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("ColorSensor::create"));
        }
    }

    uint64_t connectOnNewFrameForAsync(
        const std::shared_ptr<ColorSensor>& sensor,
        void* rgbFrameSender
    ) {
        if (!sensor) {
            throw std::runtime_error("ColorSensor instance is null in connectOnNewFrameForAsync");
        }
        try {
            return sensor->connectOnNewFrame(
                [rgbFrameSender](tdv::nuitrack::RGBFrame::Ptr frame) {
                    // CXX handles exceptions from Rust here by converting panics to rust::Error
                    rust_color_sensor_callback_which_sends_for_async(frame, rgbFrameSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("ColorSensor::connectOnNewFrame", e.what()));
        } catch (const std::exception& e) { // Catches rust::Error too
            throw std::runtime_error(format_std_error("ColorSensor::connectOnNewFrame", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("ColorSensor::connectOnNewFrame"));
        }
    }

    void disconnectOnNewFrame(
        const std::shared_ptr<ColorSensor>& sensor,
        uint64_t handlerId
    ) {
        if (!sensor) {
            throw std::runtime_error("ColorSensor instance is null in disconnectOnNewFrame");
        }
        try {
            sensor->disconnectOnNewFrame(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("ColorSensor::disconnectOnNewFrame", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("ColorSensor::disconnectOnNewFrame", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("ColorSensor::disconnectOnNewFrame"));
        }
    }

    OutputMode getOutputMode(const std::shared_ptr<ColorSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("ColorSensor instance is null in getOutputMode");
        }
        try {
            // 1. Get the OutputMode struct from the Nuitrack SDK.
            //    sensor->getOutputMode() returns a tdv::nuitrack::OutputMode by value.
            CPPOutputMode sdk_output_mode = sensor->getOutputMode();

            // 2. Create an instance of the CXX-bridged C++ struct.
            //    This is 'nuitrack_bridge::output_mode::OutputMode'.
            OutputMode cxx_bridged_mode;

            // 3. Copy the fields from the SDK struct to the CXX-bridged struct.
            cxx_bridged_mode.fps = sdk_output_mode.fps;
            cxx_bridged_mode.xres = sdk_output_mode.xres;
            cxx_bridged_mode.yres = sdk_output_mode.yres;
            cxx_bridged_mode.hfov = sdk_output_mode.hfov;

            // Copy the nested Intrinsics struct fields
            cxx_bridged_mode.intrinsics.fx = sdk_output_mode.intrinsics.fx;
            cxx_bridged_mode.intrinsics.fy = sdk_output_mode.intrinsics.fy;
            cxx_bridged_mode.intrinsics.cx = sdk_output_mode.intrinsics.cx;
            cxx_bridged_mode.intrinsics.cy = sdk_output_mode.intrinsics.cy;

            // 4. Return the populated CXX-bridged struct.
            //    CXX will then marshal this to the corresponding Rust struct.
            return cxx_bridged_mode;

        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("ColorSensor::getOutputMode", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("ColorSensor::getOutputMode", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("ColorSensor::getOutputMode"));
        }
    }

    std::shared_ptr<RGBFrame> getColorFrame(const std::shared_ptr<ColorSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("ColorSensor instance is null in getColorFrame");
        }
        try {
            return sensor->getColorFrame();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("ColorSensor::getColorFrame", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("ColorSensor::getColorFrame", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("ColorSensor::getColorFrame"));
        }
    }

    uint64_t getSensorTimestamp(const std::shared_ptr<ColorSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("ColorSensor instance is null in getSensorTimestamp");
        }
        try {
            return sensor->getTimestamp(); // This is tdv::nuitrack::Module::getTimestamp
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("ColorSensor::getTimestamp (Module)", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("ColorSensor::getTimestamp (Module)", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("ColorSensor::getTimestamp (Module)"));
        }
    }

    bool canUpdate(const std::shared_ptr<ColorSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("ColorSensor instance is null in canUpdate");
        }
        try {
            return sensor->canUpdate(); // This is tdv::nuitrack::Module::canUpdate
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("ColorSensor::canUpdate (Module)", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("ColorSensor::canUpdate (Module)", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("ColorSensor::canUpdate (Module)"));
        }
    }

} // namespace nuitrack_bridge::color_sensor