#include "nuitrack_bridge/types/depth_frame.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/depth_frame.rs.h"
#include "nuitrack/types/DepthFrame.h"
#include "nuitrack/types/Frame.h"

namespace nuitrack_bridge::depth_frame {

    // --- DepthFrame functions ---
    int32_t getRows(const DepthFrame& frame) {
        return static_cast<int32_t>(frame.getRows());
    }

    int32_t getCols(const DepthFrame& frame) {
        return static_cast<int32_t>(frame.getCols());
    }

    uint64_t getID(const DepthFrame& frame) {
        return frame.getID();
    }

    uint64_t getTimestamp(const DepthFrame& frame) {
        return frame.getTimestamp();
    }

    rust::Slice<const uint16_t> getData(const DepthFrame& frame) {
        const uint16_t* data_ptr = frame.getData();
        size_t num_pixels = static_cast<size_t>(frame.getRows()) * static_cast<size_t>(frame.getCols());
        return rust::Slice<const uint16_t>{data_ptr, num_pixels};
    }
}