#![allow(dead_code)] // TODO: remove me
#![allow(clippy::ptr_arg)] // cxx doesn't support &[T] yet.

pub use crate::backend::tensorflow::tfwrapper::ffi::SessionConfig;
use cxx::UniquePtr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("TensorFlowError: {0}")]
    TensorFlow(#[from] cxx::Exception),
}

pub struct Session {
    p: UniquePtr<ffi::Session>,
}

impl Session {
    pub fn new(config: SessionConfig) -> Result<Session, Error> {
        ffi::CreateSession(config)
            .map(|p| Session { p })
            .map_err(|e| e.into())
    }

    pub fn hello(&self) -> usize {
        self.p.Hello()
    }

    pub fn input_tensor(&self) -> Tensor {
        Tensor {
            p: self.p.InputTensor(),
        }
    }

    pub fn forward(&mut self, batch_size: usize) -> Result<Tensor, Error> {
        self.p
            .Forward(batch_size)
            .map(|p| Tensor { p })
            .map_err(|e| e.into())
    }
}

pub struct Tensor {
    p: UniquePtr<ffi::Tensor>,
}

impl Tensor {
    pub fn at(&self, index: usize) -> Tensor {
        Tensor {
            p: self.p.At(index),
        }
    }

    pub fn copy_from(&mut self, src: &Vec<f32>) -> Result<(), Error> {
        self.p.CopyFrom(src).map_err(|e| e.into())
    }

    pub fn read(&self) -> Result<Vec<f32>, Error> {
        // TODO: Avoid copying from C++ to Rust.
        self.p
            .Read()
            .map(|p| p.as_slice().into())
            .map_err(|e| e.into())
    }
}

#[cxx::bridge]
mod ffi {
    pub struct SessionConfig {
        model_pb: String,
        max_batch: usize,
        input_shape: Vec<usize>,
        input_name: String,
        output_name: String,
    }

    extern "C" {
        include!("symphony/src/backend/tensorflow/tfwrapper.h");

        type Tensor;
        pub fn At(self: &Tensor, index: usize) -> UniquePtr<Tensor>;
        pub fn CopyFrom(self: &mut Tensor, src: &Vec<f32>) -> Result<()>;
        pub fn Read(self: &Tensor) -> Result<UniquePtr<CxxVector<f32>>>;

        type Session;
        pub fn CreateSession(config: SessionConfig) -> Result<UniquePtr<Session>>;
        pub fn Hello(self: &Session) -> usize;
        pub fn InputTensor(self: &Session) -> UniquePtr<Tensor>;
        pub fn Forward(self: &mut Session, batch_size: usize) -> Result<UniquePtr<Tensor>>;
    }
}
