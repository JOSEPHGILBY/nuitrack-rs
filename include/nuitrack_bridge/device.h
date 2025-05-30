#pragma once
#include "rust/cxx.h"
#include <memory>

#include "nuitrack/types/NuitrackDevice.h"

namespace nuitrack_bridge::device {
    using Device = tdv::nuitrack::device::NuitrackDevice;
    using DeviceInfoType = tdv::nuitrack::device::DeviceInfoType;
    using SharedPtrDevice = std::shared_ptr<Device>;

    std::shared_ptr<Device> unwrapSharedPtrDevice(const SharedPtrDevice& spd);

    std::unique_ptr<std::vector<std::shared_ptr<Device>>> getDevices();

    rust::String getDeviceInfo(
        const std::shared_ptr<Device>& device_shared_ptr, // This is const std::shared_ptr<Device>&
        DeviceInfoType info_type
    );

    void setDevice(
        const std::shared_ptr<Device>& device_shared_ptr // This is const std::shared_ptr<Device>&
    );

}