use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("TensorFlowError: {0}")]
    TensorFlow(#[from] cxx::Exception),

    #[error("CudaError: {0}")]
    Cuda(String)
}
