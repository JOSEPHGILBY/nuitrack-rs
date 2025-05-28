#include "nuitrack/Nuitrack.h"
#include "nuitrack_bridge/device.h"
#include <stdexcept>
#include <string>

namespace nuitrack_bridge::device {

    std::unique_ptr<DeviceList> get_nuitrack_device_list() {
        // Create the wrapper struct on the heap
        auto list_wrapper = std::make_unique<DeviceList>();
        try {
            // Call the static Nuitrack method and store the result in the wrapper
            list_wrapper->devices = tdv::nuitrack::Nuitrack::getDeviceList();
        } catch (const tdv::nuitrack::Exception& e) {
             throw std::runtime_error(std::string("Nuitrack getDeviceList failed: ") + e.what());
        } catch (const std::exception& e) { throw; } catch (...) { throw std::runtime_error("Unknown exception during Nuitrack getDeviceList"); }
        // Return the owning pointer to the wrapper
        return list_wrapper;
    }

    // Implementation for the list size accessor
    // Name matches #[cxx_name = "size"] in Rust bridge
    size_t DeviceList::size() const {
        return this->devices.size();
    }

    std::shared_ptr<Device> DeviceList::get(size_t index) const {
        if (index >= this->devices.size()) {
            throw std::out_of_range("NuitrackDeviceList index out of range");
        }

        return this->devices[index];
    }

    rust::String get_device_info_wrapper(
        const tdv::nuitrack::device::NuitrackDevice& device, 
        tdv::nuitrack::device::DeviceInfoType info_type
    ) {
        std::string result_std_string; // Temporary std::string
        try {
            // WARNING: Assuming getInfo is logically const despite missing C++ const qualifier.
            result_std_string = const_cast<tdv::nuitrack::device::NuitrackDevice&>(device).getInfo(info_type);
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

    void nuitrack_set_device_wrapper(std::shared_ptr<Device> device) {
        try {
            // Call the static Nuitrack::setDevice method, passing the shared_ptr
            tdv::nuitrack::Nuitrack::setDevice(device);
        } catch (const tdv::nuitrack::Exception& e) {
            // Propagate exceptions for Rust Result handling
            throw std::runtime_error(std::string("Nuitrack setDevice failed: ") + e.what());
        } catch (const std::exception& e) {
            throw; // Rethrow other standard exceptions
        } catch (...) {
            throw std::runtime_error("Unknown exception during Nuitrack setDevice");
        }
        // No return on success (void maps to Ok(()))
    }
}