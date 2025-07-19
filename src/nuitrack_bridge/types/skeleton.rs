#[cxx::bridge(namespace = "nuitrack_bridge::skeleton")]
pub mod ffi {

    #[namespace = "nuitrack_bridge::vector3"]
    unsafe extern "C++" {
        // We're telling cxx that the Rust type `Vector3` corresponds to a type
        // from the `nuitrack_bridge::vector3` C++ namespace. We provide the
        // full Rust path so the Rust compiler can find its definition.
        type Vector3 = crate::nuitrack_bridge::types::vector3::ffi::Vector3;
    }


    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum JointType {
        #[cxx_name = "JOINT_NONE"]
        None = 0,            // Reserved joint (unused).
        #[cxx_name = "JOINT_HEAD"]
        Head = 1,
        #[cxx_name = "JOINT_NECK"]
        Neck = 2,
        #[cxx_name = "JOINT_TORSO"]
        Torso = 3,
        #[cxx_name = "JOINT_WAIST"]
        Waist = 4,
        #[cxx_name = "JOINT_LEFT_COLLAR"]
        LeftCollar = 5,
        #[cxx_name = "JOINT_LEFT_SHOULDER"]
        LeftShoulder = 6,
        #[cxx_name = "JOINT_LEFT_ELBOW"]
        LeftElbow = 7,
        #[cxx_name = "JOINT_LEFT_WRIST"]
        LeftWrist = 8,
        #[cxx_name = "JOINT_LEFT_HAND"]
        LeftHand = 9,
        #[cxx_name = "JOINT_LEFT_FINGERTIP"]
        LeftFingertip = 10,  // Left fingertip (not used in the current version by Nuitrack).
        #[cxx_name = "JOINT_RIGHT_COLLAR"]
        RightCollar = 11,
        #[cxx_name = "JOINT_RIGHT_SHOULDER"]
        RightShoulder = 12,
        #[cxx_name = "JOINT_RIGHT_ELBOW"]
        RightElbow = 13,
        #[cxx_name = "JOINT_RIGHT_WRIST"]
        RightWrist = 14,
        #[cxx_name = "JOINT_RIGHT_HAND"]
        RightHand = 15,
        #[cxx_name = "JOINT_RIGHT_FINGERTIP"]
        RightFingertip = 16, // Right fingertip (not used in the current version by Nuitrack).
        #[cxx_name = "JOINT_LEFT_HIP"]
        LeftHip = 17,
        #[cxx_name = "JOINT_LEFT_KNEE"]
        LeftKnee = 18,
        #[cxx_name = "JOINT_LEFT_ANKLE"]
        LeftAnkle = 19,
        #[cxx_name = "JOINT_LEFT_FOOT"]
        LeftFoot = 20,       // Left foot (not used in the current version by Nuitrack).
        #[cxx_name = "JOINT_RIGHT_HIP"]
        RightHip = 21,
        #[cxx_name = "JOINT_RIGHT_KNEE"]
        RightKnee = 22,
        #[cxx_name = "JOINT_RIGHT_ANKLE"]
        RightAnkle = 23,
        #[cxx_name = "JOINT_RIGHT_FOOT"]
        RightFoot = 24,      // Right foot (not used in the current version by Nuitrack).
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Orientation {
        pub matrix: [f32; 9],
    }

    #[derive(Debug, Clone, Copy)] 
    pub struct Joint {
        pub joint_type: JointType,
        pub confidence: f32,
        pub real: Vector3,
        pub proj: Vector3,
        pub orient: Orientation

    }


    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type Skeleton;
    }

    impl CxxVector<Skeleton> {}

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/skeleton.h");

        #[cxx_name = "getUserID"]
        pub fn user_id(skeleton: &Skeleton) -> i32;
        
        #[cxx_name = "getJoints"]
        pub fn joints<'a>(skeleton: &'a Skeleton) -> &'a [Joint];

    }
}