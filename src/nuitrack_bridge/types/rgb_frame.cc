#include "nuitrack_bridge/types/rgb_frame.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/rgb_frame.rs.h"
#include "nuitrack/types/RGBFrame.h"
#include "nuitrack/types/Color3.h"
#include "nuitrack/types/Frame.h"

namespace nuitrack_bridge::rgb_frame {

    // --- RGBFrame functions ---
    int32_t getRows(const RGBFrame& frame) {
        return static_cast<int32_t>(frame.getRows());
    }

    int32_t getCols(const RGBFrame& frame) {
        return static_cast<int32_t>(frame.getCols());
    }

    uint64_t getID(const RGBFrame& frame) {
        return frame.getID();
    }

    uint64_t getTimestamp(const RGBFrame& frame) {
        return frame.getTimestamp();
    }

    rust::Slice<const Color3> getData(const RGBFrame& frame) {
        const tdv::nuitrack::Color3* data_ptr = frame.getData();
        size_t num_pixels = static_cast<size_t>(frame.getRows()) * static_cast<size_t>(frame.getCols());
        return rust::Slice<const Color3>{reinterpret_cast<const Color3*>(data_ptr), num_pixels};
    }
}