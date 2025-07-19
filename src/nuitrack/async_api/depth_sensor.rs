use crate::nuitrack::shared_types::depth_frame::DepthFrame;
use crate::nuitrack_bridge::modules::depth_sensor::ffi as depth_sensor_ffi;
use crate::nuitrack_bridge::types::{
    output_mode::ffi::OutputMode as OutputModeFfi,
    vector3::ffi::Vector3 as Vector3Ffi
};
use tracing::warn;

// This macro likely generates the boilerplate for creating the sensor,
// handling callbacks, and creating a data stream.
generate_async_tracker! {
    base_module_name_snake: depth_sensor,
    module_ffi_path: crate::nuitrack_bridge::modules::depth_sensor::ffi,
    streams: [
        {
            item_base_name_snake: depth_frame,
            item_base_name_pascal: DepthFrame,
            rust_item_type: crate::nuitrack::shared_types::depth_frame::DepthFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: frame,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::depth_frame::ffi::DepthFrame>,
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::depth_frame::ffi::DepthFrame>| {
                    crate::nuitrack::shared_types::depth_frame::DepthFrame::new(data_arg.clone())
                },
            }}
        }
    ]
}

impl AsyncDepthSensor {
    /// Gets the current output mode of the depth sensor.
    #[instrument(skip(self))]
    pub async fn output_mode(&self) -> NuitrackResult<OutputModeFfi> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "depth_sensor_ffi::output_mode").in_scope(
            || {
                run_blocking(move || {
                    depth_sensor_ffi::output_mode(&ptr).map_err(|e| {
                        NuitrackError::OperationFailed(format!("Failed to get output mode: {}", e))
                    })
                })
            },
        )
        .await
    }

    /// Gets the last available depth frame synchronously from the sensor.
    #[instrument(skip(self))]
    pub async fn latest_depth_frame_sync(&self) -> NuitrackResult<DepthFrame> {
        let ptr = self.get_ffi_ptr_clone();
        let ffi_depth_frame_ptr = trace_span!("ffi", function = "depth_sensor_ffi::depth_frame")
            .in_scope(|| {
                run_blocking(move || {
                    depth_sensor_ffi::depth_frame(&ptr).map_err(|e| {
                        NuitrackError::OperationFailed(format!(
                            "Failed to get depth frame synchronously: {}",
                            e
                        ))
                    })
                })
            })
            .await?;

        DepthFrame::new(ffi_depth_frame_ptr).ok_or_else(|| {
            warn!("FFI call for latest depth frame returned a null pointer.");
            NuitrackError::OperationFailed(
                "Received null DepthFrame from get_depth_frame_sync".to_string(),
            )
        })
    }

    /// Checks if mirror mode is enabled.
    #[instrument(skip(self))]
    pub async fn is_mirror(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "depth_sensor_ffi::is_mirror").in_scope(
            || {
                run_blocking(move || {
                    depth_sensor_ffi::is_mirror(&ptr).map_err(|e| {
                        NuitrackError::OperationFailed(format!("Failed to get mirror status: {}", e))
                    })
                })
            },
        )
        .await
    }

    /// Sets the mirror mode.
    #[instrument(skip(self))]
    pub async fn set_mirror(&self, mirror: bool) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "depth_sensor_ffi::set_mirror").in_scope(
            || {
                run_blocking(move || {
                    depth_sensor_ffi::set_mirror(&ptr, mirror).map_err(|e| {
                        NuitrackError::OperationFailed(format!("Failed to set mirror status: {}", e))
                    })
                })
            },
        )
        .await
    }

    /// Converts projective coordinates to real-world coordinates.
    #[instrument(skip(self))]
    pub async fn convert_proj_to_real(
        &self,
        point: Vector3Ffi,
    ) -> NuitrackResult<Vector3Ffi> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "depth_sensor_ffi::convert_proj_to_real_coords")
            .in_scope(move || {
                run_blocking(move || {
                    depth_sensor_ffi::convert_proj_to_real_coords(&ptr, &point).map_err(|e| {
                        NuitrackError::OperationFailed(format!(
                            "Failed to convert proj to real coords: {}",
                            e
                        ))
                    })
                })
            })
            .await
    }

    /// Converts real-world coordinates to projective coordinates.
    #[instrument(skip(self))]
    pub async fn convert_real_to_proj(
        &self,
        point: Vector3Ffi,
    ) -> NuitrackResult<Vector3Ffi> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "depth_sensor_ffi::convert_real_to_proj_coords")
            .in_scope(move || {
                run_blocking(move || {
                    depth_sensor_ffi::convert_real_to_proj_coords(&ptr, &point).map_err(|e| {
                        NuitrackError::OperationFailed(format!(
                            "Failed to convert real to proj coords: {}",
                            e
                        ))
                    })
                })
            })
            .await
    }

    /// Gets the timestamp of the last processed data by the depth sensor in microseconds.
    #[instrument(skip(self))]
    pub async fn sensor_timestamp(&self) -> NuitrackResult<u64> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "depth_sensor_ffi::timestamp").in_scope(
            || {
                run_blocking(move || {
                    depth_sensor_ffi::timestamp(&ptr).map_err(|e| {
                        NuitrackError::OperationFailed(format!("Failed to get sensor timestamp: {}", e))
                    })
                })
            },
        )
        .await
    }

    /// Checks if the Nuitrack depth sensor module can update.
    #[instrument(skip(self))]
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "depth_sensor_ffi::can_update").in_scope(
            || {
                run_blocking(move || {
                    depth_sensor_ffi::can_update(&ptr).map_err(|e| {
                        NuitrackError::OperationFailed(format!(
                            "Failed to check can_update status: {}",
                            e
                        ))
                    })
                })
            },
        )
        .await
    }
}