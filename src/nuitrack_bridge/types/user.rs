

#[cxx::bridge(namespace = "nuitrack_bridge::user")]
pub mod ffi {

    #[namespace = "nuitrack_bridge::vector3"]
    unsafe extern "C++" {
        type Vector3 = crate::nuitrack_bridge::types::vector3::ffi::Vector3;
    }
    
    #[namespace = "nuitrack_bridge::bounding_box"]
    unsafe extern "C++" {
        type BoundingBox = crate::nuitrack_bridge::types::bounding_box::ffi::BoundingBox;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct User {
        pub id: i32,
        pub proj: Vector3,
        pub real: Vector3,
        #[cxx_name = "box"]
        pub r#box: BoundingBox,
        pub occlusion: f32,
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/user.h");
    }
}
