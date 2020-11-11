mod backend;
mod common;

#[allow(dead_code)]
mod msg_capnp {
    include!(concat!(env!("OUT_DIR"), "/src/msg_capnp.rs"));
}
