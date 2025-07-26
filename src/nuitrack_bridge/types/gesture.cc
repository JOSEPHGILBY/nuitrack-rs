#include "nuitrack_bridge/types/gesture.h"
#include "nuitrack/types/Gesture.h"
#include "nuitrack-rs/src/nuitrack_bridge/types/gesture.rs.h"


namespace nuitrack_bridge::gesture {

    int32_t getUserID(const UserGesturesState& state) {
        return state.userId;
    }

    UserStateType getUserState(const UserGesturesState& state) {
        // The Rust enum and C++ enum will have the same underlying integer values.
        return static_cast<UserStateType>(state.state);
    }

    rust::Slice<const GestureState> getGestures(const UserGesturesState& state) {
        // Create a slice from the vector of GestureState
        const auto& gestures_vec = state.gestures;
        return rust::Slice<const GestureState>{
            reinterpret_cast<const GestureState*>(gestures_vec.data()),
            gestures_vec.size()
        };
    }

}