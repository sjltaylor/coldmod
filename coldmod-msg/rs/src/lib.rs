pub mod proto {
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/", "coldmod_msg.proto.trace", ".rs")
    ));
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/", "coldmod_msg.proto.source", ".rs")
    ));
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/", "coldmod_msg.proto.ops", ".rs")
    ));
}

pub mod web;
