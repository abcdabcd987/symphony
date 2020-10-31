#![allow(dead_code)] // TODO: remove me

use cxx::UniquePtr;
use thiserror::Error;

pub struct Session {
    p: UniquePtr<ffi::Session>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("TensorFlowError: {0}")]
    TensorFlow(#[from] cxx::Exception),
}

impl Session {
    pub fn new(model_pb: &str) -> Result<Session, Error> {
        ffi::CreateSession(model_pb)
            .map(|p| Session { p })
            .map_err(|e| e.into())
    }

    pub fn hello(&self) -> usize {
        self.p.Hello()
    }
}

#[cxx::bridge]
mod ffi {
    extern "C" {
        include!("symphony/src/backend/tensorflow/tfwrapper.h");
        type Session;
        pub fn CreateSession(model_pb: &str) -> Result<UniquePtr<Session>>;
        pub fn Hello(self: &Session) -> usize;
    }
}
