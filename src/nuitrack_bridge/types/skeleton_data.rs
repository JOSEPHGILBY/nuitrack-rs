#[cxx::bridge(namespace = "nuitrack_bridge::skeleton_data")]
pub mod ffi {

    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type SkeletonData;
        type Skeleton = crate::nuitrack_bridge::types::skeleton::ffi::Skeleton;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/skeleton_data.h");
        
        #[cxx_name = "getNumSkeletons"]
        pub fn get_num_skeletons(skeleton_frame: &SkeletonData) -> Result<i32>;

        #[cxx_name = "getSkeletons"]
        pub fn get_skeletons(skeleton_frame: &SkeletonData) -> Result<UniquePtr<CxxVector<Skeleton>>>;

        #[cxx_name = "getTimestamp"]
        pub fn get_timestamp(skeleton_frame: &SkeletonData) -> Result<u64>;

        #[cxx_name = "doNotUseMakeSharedPtrAware"]
        fn do_not_use_make_shared_ptr_aware(data: &SharedPtr<SkeletonData>);
    }
}

unsafe impl Send for ffi::SkeletonData {}
unsafe impl Sync for ffi::SkeletonData {}