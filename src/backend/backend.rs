use crate::msg_capnp;
use crate::common;
use std::collections::HashMap;

pub struct Backend {
}

impl Backend {
    pub fn handle_backend_message(&self, message: msg_capnp::backend_message::Reader) {
        match message.which() {
            Ok(msg_capnp::backend_message::Load(Ok(reader))) => {
                self.handle_load(reader);
            }
            Ok(msg_capnp::backend_message::Images(Ok(reader))) => {
                self.handle_images(reader);
            }
            Err(::capnp::NotInSchema(_)) => {
                panic!("not in schema");
            }
            _ => {
                panic!("other error");
            }
        }
    }

    pub fn handle_load(&self, reader: msg_capnp::load_model_session_command::Reader) {
        todo!();
    }

    pub fn handle_images(&self, reader: msg_capnp::read_images_rpc_reply::Reader) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capnp_test() {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut message = capnp::message::Builder::new_default();
            let backend_message = message.init_root::<msg_capnp::backend_message::Builder>();
            let load_cmd = backend_message.init_load();
            let mut model_session = load_cmd.init_model_session();
            model_session.set_framework("tensorflow");
            model_session.set_model("resnet");
            model_session.set_slo_ms(100);
            capnp::serialize::write_message(&mut buf, &message).unwrap();
        }
        {
            let cursor = std::io::Cursor::new(&buf);
            let reader =
                capnp::serialize::read_message(cursor, capnp::message::ReaderOptions::new())
                    .unwrap();
            let backend_message = reader
                .get_root::<msg_capnp::backend_message::Reader>()
                .unwrap();
            match backend_message.which() {
                Ok(msg_capnp::backend_message::Load(Ok(load))) => {
                    let model_session = load.get_model_session().unwrap();
                    assert_eq!(model_session.get_framework().unwrap(), "tensorflow");
                }
                _ => {
                    panic!();
                }
            }
        }
        {
            let cursor = std::io::Cursor::new(&buf);
            let reader: capnp::message::TypedReader<_, msg_capnp::backend_message::Owned> =
                capnp::serialize::read_message(cursor, capnp::message::ReaderOptions::new())
                    .unwrap().into_typed();
            let backend_message = reader.get().unwrap();;
            match backend_message.which() {
                Ok(msg_capnp::backend_message::Load(Ok(load))) => {
                    let model_session = load.get_model_session().unwrap();
                    assert_eq!(model_session.get_framework().unwrap(), "tensorflow");
                }
                _ => {
                    panic!();
                }
            }
        }
    }
}
