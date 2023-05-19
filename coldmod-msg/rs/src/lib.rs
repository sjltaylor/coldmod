#[cfg(feature = "proto")]
pub mod proto {
    tonic::include_proto!("coldmod_msg.proto"); // The string specified here must match the proto package name
}

pub mod web;
