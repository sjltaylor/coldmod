#[allow(non_camel_case_types)]
pub mod proto {
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/", "coldmod_msg.proto.tracing", ".rs")
    ));
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/", "coldmod_msg.proto.ops", ".rs")
    ));
}

pub mod web;
