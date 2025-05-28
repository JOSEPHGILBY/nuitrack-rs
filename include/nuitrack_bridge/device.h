#pragma once
#include "rust/cxx.h"
#include <memory>

#include "nuitrack/types/NuitrackDevice.h"

namespace nuitrack_bridge::device {
    using Device = tdv::nuitrack::device::NuitrackDevice;
    using DeviceInfoType = tdv::nuitrack::device::DeviceInfoType;

    struct DeviceList {
        // Holds the actual vector that the static Nuitrack method returns
        std::vector<std::shared_ptr<Device>> devices;

        size_t size() const;
        std::shared_ptr<Device> get(size_t index) const;
    };

    std::unique_ptr<DeviceList> get_nuitrack_device_list();

    rust::String get_device_info_wrapper(
        const Device& device, 
        DeviceInfoType info_type       
    );

    void nuitrack_set_device_wrapper(std::shared_ptr<Device> device);
}