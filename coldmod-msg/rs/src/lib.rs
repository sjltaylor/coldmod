// #[cfg(feature = "proto")]
// pub mod proto {
//     // The string specified here must match the proto package name
//     tonic::include_proto!("coldmod_msg.proto.trace");
//     tonic::include_proto!("coldmod_msg.proto.source");
// }

pub mod proto {
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/", "coldmod_msg.proto.trace", ".rs")
    ));
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/", "coldmod_msg.proto.source", ".rs")
    ));
}

pub mod web;
