#[cfg(not(feature = "serde"))]
#[cxx::bridge(namespace = "nuitrack_bridge::vector3")]
pub mod ffi {
    /// A CXX-compatible struct representing a 3D vector.
    ///
    /// This struct is defined in Rust and shared with C++. Its memory layout
    /// is identical to the tdv::nuitrack::Vector3, allowing for safe conversions.
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
}

#[cfg(feature = "serde")]
#[cxx::bridge(namespace = "nuitrack_bridge::vector3")]
pub mod ffi {

    #[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
}
