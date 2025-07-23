// File: /validation_apps/test-serde-feature-builds/src/main.rs

// We need to import one of the types that has the conditional serde derive.
// Vector3 is a perfect, simple choice.
use nuitrack_rs::nuitrack::async_api::depth_sensor::Vector3;

fn main() {
    // The purpose of this test is to ensure that the project compiles
    // when the "serde" feature is enabled. We don't need complex logic.

    // Just creating an instance of the struct is enough for the compiler
    // to check that the `Serialize` and `Deserialize` traits are correctly derived.
    let _vector = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    // A print statement confirms that the build and run were successful.
    println!("Successfully compiled and ran the serde feature validation app!");
    println!("This confirms that `nuitrack-rs` builds correctly with the 'serde' feature enabled.");
}