use super::error::BackendError;
use std::ffi::CStr;

pub struct CudaDevice {
    device_id: i32,
    name: String,
    uuid: String,
}

unsafe fn unwrap(error: cuda_runtime_sys::cudaError_t) -> Result<(), BackendError> {
    if error == cuda_runtime_sys::cudaError_t::cudaSuccess {
        Ok(())
    } else {
        let cstr = CStr::from_ptr(cuda_runtime_sys::cudaGetErrorString(error));
        Err(BackendError::Cuda(cstr.to_string_lossy().to_string()))
    }
}

impl CudaDevice {
    pub fn new(device_id: i32) -> Result<CudaDevice, BackendError> {
        unsafe {
            unwrap(cuda_runtime_sys::cudaSetDevice(device_id))?;

            let mut prop = cuda_runtime_sys::cudaDeviceProp::default();
            unwrap(cuda_runtime_sys::cudaGetDeviceProperties(
                &mut prop as *mut _,
                device_id,
            ))?;

            let name = CStr::from_ptr(&prop.name as *const _)
                .to_string_lossy()
                .to_string();
            let u = &prop.uuid.bytes;
            let uuid = format!(
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                u[0], u[1], u[2], u[3], u[4], u[5], u[6], u[7], u[8], u[9], u[10],
                u[11], u[12], u[13], u[14], u[15]);
            Ok(CudaDevice {
                device_id,
                name,
                uuid,
            })
        }
    }

    pub fn device_id(&self) -> i32 {
        self.device_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }
}
