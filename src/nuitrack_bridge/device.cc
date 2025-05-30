#include "nuitrack/Nuitrack.h"
#include "nuitrack_bridge/device.h"
#include <stdexcept>
#include <string>

namespace nuitrack_bridge::device {

    std::shared_ptr<Device> unwrapSharedPtrDevice(const SharedPtrDevice& spd) {
        return spd;
    }

    std::unique_ptr<std::vector<std::shared_ptr<Device>>> getDevices() {
        return std::make_unique<std::vector<std::shared_ptr<tdv::nuitrack::device::NuitrackDevice>>>(
            tdv::nuitrack::Nuitrack::getDeviceList()
        );
    }

    rust::String getDeviceInfo(
        const std::shared_ptr<Device>& device_shared_ptr, // This is const std::shared_ptr<Device>&
        tdv::nuitrack::device::DeviceInfoType info_type
    ) {
        if (!device_shared_ptr) {
            throw std::runtime_error("Device (SharedPtrDevice) is null in get_device_info_from_shared");
        }
        
        std::string result_std_string; // Temporary std::string
        try {
            // WARNING: Assuming getInfo is logically const despite missing C++ const qualifier.
            result_std_string = const_cast<tdv::nuitrack::device::NuitrackDevice&>(*device_shared_ptr).getInfo(info_type);
        } catch (const tdv::nuitrack::Exception& e) {
            // Propagate exceptions for Rust's Result handling
             throw std::runtime_error(std::string("Nuitrack getInfo failed: ") + e.what());
        } catch (const std::exception& e) {
             throw; // Rethrow other std exceptions
        } catch (...) {
             throw std::runtime_error("Unknown exception during Nuitrack getInfo");
        }
        // *** Convert std::string to rust::String before returning ***
        return rust::String(result_std_string);
    }

    void setDevice(
        const std::shared_ptr<Device>& device_shared_ptr // This is const std::shared_ptr<Device>&
    ) {
        if (!device_shared_ptr) {
            // Or handle as Nuitrack::setDevice would (it might accept null to unset).
            // For safety, let's assume we don't want to pass a null pointer if it's not explicitly supported.
             throw std::runtime_error("Device (SharedPtrDevice) is null in set_device_from_shared. Cannot set null device unless Nuitrack::setDevice explicitly supports it.");
        }
        try {
            // Call the static Nuitrack::setDevice method, passing the shared_ptr
            tdv::nuitrack::Nuitrack::setDevice(device_shared_ptr);
        } catch (const tdv::nuitrack::Exception& e) {
            // Propagate exceptions for Rust Result handling
            throw std::runtime_error(std::string("Nuitrack setDevice failed: ") + e.what());
        } catch (const std::exception& e) {
            throw; // Rethrow other standard exceptions
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack setDevice");
        }
    }
}