#[cfg(not(feature = "serde"))]
#[cxx::bridge(namespace = "nuitrack_bridge::skeleton")]
pub mod ffi {

    #[namespace = "nuitrack_bridge::vector3"]
    unsafe extern "C++" {
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


#[cfg(feature = "serde")]
#[cxx::bridge(namespace = "nuitrack_bridge::skeleton")]
pub mod ffi {

    #[namespace = "nuitrack_bridge::vector3"]
    unsafe extern "C++" {
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

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Orientation {
        pub matrix: [f32; 9],
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")] 
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


#[cfg(feature = "serde")]
mod serde_impls {
    use super::ffi;
    use serde::{de::{self, Visitor}, Deserializer, Serializer};

    // --- IMPLEMENTATION FOR SERIALIZE ---
    impl serde::Serialize for ffi::JointType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Match on the internal integer and serialize the correct string
            match self.repr {
                0 => serializer.serialize_str("none"),
                1 => serializer.serialize_str("head"),
                2 => serializer.serialize_str("neck"),
                3 => serializer.serialize_str("torso"),
                4 => serializer.serialize_str("waist"),
                5 => serializer.serialize_str("leftCollar"),
                6 => serializer.serialize_str("leftShoulder"),
                7 => serializer.serialize_str("leftElbow"),
                8 => serializer.serialize_str("leftWrist"),
                9 => serializer.serialize_str("leftHand"),
                10 => serializer.serialize_str("leftFingertip"),
                11 => serializer.serialize_str("rightCollar"),
                12 => serializer.serialize_str("rightShoulder"),
                13 => serializer.serialize_str("rightElbow"),
                14 => serializer.serialize_str("rightWrist"),
                15 => serializer.serialize_str("rightHand"),
                16 => serializer.serialize_str("rightFingertip"),
                17 => serializer.serialize_str("leftHip"),
                18 => serializer.serialize_str("leftKnee"),
                19 => serializer.serialize_str("leftAnkle"),
                20 => serializer.serialize_str("leftFoot"),
                21 => serializer.serialize_str("rightHip"),
                22 => serializer.serialize_str("rightKnee"),
                23 => serializer.serialize_str("rightAnkle"),
                24 => serializer.serialize_str("rightFoot"),
                _ => serializer.serialize_str("unknown"),
            }
        }
    }

    // --- IMPLEMENTATION FOR DESERIALIZE ---
    impl<'de> serde::Deserialize<'de> for ffi::JointType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct JointTypeVisitor;

            impl<'de> Visitor<'de> for JointTypeVisitor {
                type Value = ffi::JointType;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a joint type string like 'head' or 'rightShoulder'")
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    // Match on the incoming string and return the correct struct
                    let repr = match value {
                        "none" => 0,
                        "head" => 1,
                        "neck" => 2,
                        "torso" => 3,
                        "waist" => 4,
                        "leftCollar" => 5,
                        "leftShoulder" => 6,
                        "leftElbow" => 7,
                        "leftWrist" => 8,
                        "leftHand" => 9,
                        "leftFingertip" => 10,
                        "rightCollar" => 11,
                        "rightShoulder" => 12,
                        "rightElbow" => 13,
                        "rightWrist" => 14,
                        "rightHand" => 15,
                        "rightFingertip" => 16,
                        "leftHip" => 17,
                        "leftKnee" => 18,
                        "leftAnkle" => 19,
                        "leftFoot" => 20,
                        "rightHip" => 21,
                        "rightKnee" => 22,
                        "rightAnkle" => 23,
                        "rightFoot" => 24,
                        _ => return Err(E::unknown_variant(value, &["head", "neck", "..."])),
                    };
                    Ok(ffi::JointType { repr })
                }
            }

            deserializer.deserialize_str(JointTypeVisitor)
        }
    }
}