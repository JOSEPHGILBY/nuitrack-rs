#include "nuitrack_bridge/modules/depth_sensor.h"
#include "nuitrack-rs/src/nuitrack_bridge/modules/depth_sensor.rs.h"
#include "nuitrack/modules/DepthSensor.h"
#include "nuitrack/types/DepthFrame.h"
#include "nuitrack/types/Vector3.h"
#include "nuitrack/types/OutputMode.h"
#include "nuitrack/Nuitrack.h" 

#include <stdexcept>
#include <string>
#include <functional>

// Helper error formatting functions
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

namespace nuitrack_bridge::depth_sensor {

    std::shared_ptr<DepthSensor> createDepthSensor() {
        try {
            return tdv::nuitrack::DepthSensor::create();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("DepthSensor::create", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("DepthSensor::create", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("DepthSensor::create"));
        }
    }

    uint64_t connectOnNewFrameForAsync(
        const std::shared_ptr<DepthSensor>& sensor,
        void* depthFrameSender
    ) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in connectOnNewFrameForAsync");
        }
        try {
            return sensor->connectOnNewFrame(
                [depthFrameSender](tdv::nuitrack::DepthFrame::Ptr frame) {
                    rust_depth_sensor_depth_frame_dispatcher_async(frame, depthFrameSender);
                });
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("DepthSensor::connectOnNewFrame", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("DepthSensor::connectOnNewFrame", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("DepthSensor::connectOnNewFrame"));
        }
    }

    void disconnectOnNewFrame(
        const std::shared_ptr<DepthSensor>& sensor,
        uint64_t handlerId
    ) {
        if (!sensor) return; // Disconnecting on a null sensor is a no-op
        try {
            sensor->disconnectOnNewFrame(handlerId);
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("DepthSensor::disconnectOnNewFrame", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("DepthSensor::disconnectOnNewFrame", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("DepthSensor::disconnectOnNewFrame"));
        }
    }

    std::shared_ptr<DepthFrame> getDepthFrame(const std::shared_ptr<DepthSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in getDepthFrame");
        }
        try {
            return sensor->getDepthFrame();
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("DepthSensor::getDepthFrame", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("DepthSensor::getDepthFrame", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("DepthSensor::getDepthFrame"));
        }
    }

    OutputMode getOutputMode(const std::shared_ptr<DepthSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in getOutputMode");
        }
        try {
            CPPOutputMode sdk_output_mode = sensor->getOutputMode();
            OutputMode cxx_bridged_mode;
            cxx_bridged_mode.fps = sdk_output_mode.fps;
            cxx_bridged_mode.xres = sdk_output_mode.xres;
            cxx_bridged_mode.yres = sdk_output_mode.yres;
            cxx_bridged_mode.hfov = sdk_output_mode.hfov;
            cxx_bridged_mode.intrinsics.fx = sdk_output_mode.intrinsics.fx;
            cxx_bridged_mode.intrinsics.fy = sdk_output_mode.intrinsics.fy;
            cxx_bridged_mode.intrinsics.cx = sdk_output_mode.intrinsics.cx;
            cxx_bridged_mode.intrinsics.cy = sdk_output_mode.intrinsics.cy;
            return cxx_bridged_mode;
        } catch (const tdv::nuitrack::Exception& e) {
            throw std::runtime_error(format_nuitrack_error("DepthSensor::getOutputMode", e.what()));
        } catch (const std::exception& e) {
            throw std::runtime_error(format_std_error("DepthSensor::getOutputMode", e.what()));
        } catch (...) {
            throw std::runtime_error(format_unknown_error("DepthSensor::getOutputMode"));
        }
    }

    bool isMirror(const std::shared_ptr<DepthSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in isMirror");
        }
        return sensor->isMirror();
    }

    void setMirror(const std::shared_ptr<DepthSensor>& sensor, bool mirror) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in setMirror");
        }
        sensor->setMirror(mirror);
    }

    Vector3 convertProjToRealCoords(const std::shared_ptr<DepthSensor>& sensor, const Vector3& p) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in convertProjToRealCoords");
        }
        CPPVector3 sdk_vec_in = { p.x, p.y, p.z };
        CPPVector3 sdk_vec_out = sensor->convertProjToRealCoords(sdk_vec_in);
        return Vector3{ sdk_vec_out.x, sdk_vec_out.y, sdk_vec_out.z };
    }

    Vector3 convertRealToProjCoords(const std::shared_ptr<DepthSensor>& sensor, const Vector3& p) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in convertRealToProjCoords");
        }
        CPPVector3 sdk_vec_in = { p.x, p.y, p.z };
        CPPVector3 sdk_vec_out = sensor->convertRealToProjCoords(sdk_vec_in);
        return Vector3{ sdk_vec_out.x, sdk_vec_out.y, sdk_vec_out.z };
    }

    uint64_t getSensorTimestamp(const std::shared_ptr<DepthSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in getSensorTimestamp");
        }
        return sensor->getTimestamp();
    }

    bool canUpdate(const std::shared_ptr<DepthSensor>& sensor) {
        if (!sensor) {
            throw std::runtime_error("DepthSensor instance is null in canUpdate");
        }
        return sensor->canUpdate();
    }

}