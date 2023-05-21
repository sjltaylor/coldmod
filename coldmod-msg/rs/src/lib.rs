#[cfg(feature = "proto")]
pub mod proto {
    // The string specified here must match the proto package name
    tonic::include_proto!("coldmod_msg.proto.trace");
    tonic::include_proto!("coldmod_msg.proto.source");
}

pub mod web;
