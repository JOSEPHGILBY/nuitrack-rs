#[cxx::bridge(namespace = "nuitrack_bridge::skeleton_data")]
pub mod ffi {

    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type SkeletonData;
        type Skeleton = crate::nuitrack_bridge::types::skeleton::ffi::Skeleton;
    }

    impl SharedPtr<SkeletonData> {}

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/skeleton_data.h");
        
        #[cxx_name = "getNumSkeletons"]
        pub fn num_skeletons(skeleton_frame: &SkeletonData) -> Result<i32>;

        #[cxx_name = "getSkeletons"]
        pub fn skeletons(skeleton_frame: &SkeletonData) -> Result<UniquePtr<CxxVector<Skeleton>>>;

        #[cxx_name = "getTimestamp"]
        pub fn timestamp(skeleton_frame: &SkeletonData) -> Result<u64>;

    }
}

unsafe impl Send for ffi::SkeletonData {}
unsafe impl Sync for ffi::SkeletonData {}