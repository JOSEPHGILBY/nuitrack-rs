
#[cxx::bridge(namespace = "nuitrack_bridge::bounding_box")]
pub mod ffi {
    /// A CXX-compatible struct representing a 3D vector.
    ///
    /// This struct is defined in Rust and shared with C++. Its memory layout
    /// is identical to the tdv::nuitrack::BoundingBox, allowing for safe conversions.
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct BoundingBox {
        pub top: f32,
        pub bottom: f32,
        pub left: f32,
        pub right: f32,
    }
}
