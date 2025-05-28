#[cxx::bridge(namespace = "nuitrack_bridge::skeleton_tracker")]
pub mod ffi {

    // Opaque C++ types that Rust will interact with via pointers/references.
    #[namespace = "tdv::nuitrack"] // Map to the original Nuitrack namespace
    unsafe extern "C++" {
        type SkeletonTracker;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/skeleton_tracker.h");

        #[cxx_name = "create_skeleton_tracker"]
        pub fn skeleton_tracker_create() -> Result<SharedPtr<SkeletonTracker>>;
    }
}

unsafe impl Send for ffi::SkeletonTracker {}
unsafe impl Sync for ffi::SkeletonTracker {}