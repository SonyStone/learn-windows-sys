use windows::{
    core::Result,
    Win32::Graphics::Direct2D::{ID2D1Device, D2D1_DEVICE_CONTEXT_OPTIONS_NONE},
};

use super::device_context::DeviceContext;

/// A Direct2D device.
pub struct Direct2DDevice(pub(crate) ID2D1Device);

impl Drop for Direct2DDevice {
    fn drop(&mut self) {
        println!("🚮 Direct2DDevice dropped here")
    }
}

impl Direct2DDevice {
    /// Create a new device context from the device.
    ///
    /// This is a wrapper for
    /// [ID2D1Device::CreateDeviceContext](https://docs.microsoft.com/en-us/windows/win32/api/d2d1_1/nf-d2d1_1-id2d1device-createdevicecontext).
    ///
    /// Represents a set of state and command buffers that are used to render to a target.
    pub fn create_device_context(&self) -> Result<DeviceContext> {
        unsafe {
            let options = D2D1_DEVICE_CONTEXT_OPTIONS_NONE;
            let context = self.0.CreateDeviceContext(options)?;
            Ok(DeviceContext::new(context))
        }
    }
}
