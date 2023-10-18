use windows::{
    core::Result,
    Win32::Graphics::Dxgi::{Common::DXGI_FORMAT, IDXGISurface, IDXGISwapChain1},
};

use super::device_context::DeviceContext;

#[derive(Debug)]
pub struct DXGISwapChain(IDXGISwapChain1);

impl Drop for DXGISwapChain {
    fn drop(&mut self) {
        println!("🚮 DXGISwapChain dropped here")
    }
}

impl DXGISwapChain {
    pub fn new(swapcahin: IDXGISwapChain1) -> Self {
        DXGISwapChain(swapcahin)
    }

    pub fn create_swapchain_bitmap(&self, target: &DeviceContext) -> Result<()> {
        let surface: IDXGISurface = unsafe { self.0.GetBuffer(0)? };

        let bitmap = target.create_bitmap_from_dxgi(&surface, 1.0)?;
        target.set_target(&bitmap.inner);

        Ok(())
    }

    /// [IDXGISwapChain::Present](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-present)
    ///
    /// Presents a rendered image to the user.
    /// # Parameters
    /// `sync` - SyncInterval
    ///
    /// An integer that specifies how to synchronize presentation of a frame with the vertical blank.
    /// For the bit-block transfer (bitblt) model (DXGI_SWAP_EFFECT_DISCARD or DXGI_SWAP_EFFECT_SEQUENTIAL), values are:
    /// * 0 - The presentation occurs immediately, there is no synchronization.
    /// * 1 through 4 - Synchronize presentation after the nth vertical blank.
    ///
    /// For the flip model (DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL), values are:
    /// * 0 - Cancel the remaining time on the previously presented frame and discard this frame if a newer frame is queued.
    /// * 1 through 4 - Synchronize presentation for at least n vertical blanks.
    ///
    /// For an example that shows how sync-interval values affect a flip presentation queue, see Remarks.
    ///
    /// `Flags`
    ///
    /// An integer value that contains swap-chain presentation options. These options are defined by the DXGI_PRESENT constants.
    pub fn present(&self, sync: u32, flags: u32) -> Result<()> {
        unsafe { self.0.Present(sync, flags).ok() }
    }

    pub fn resize_buffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> Result<()> {
        unsafe {
            self.0
                .ResizeBuffers(buffercount, width, height, newformat, swapchainflags)
        }
    }
}
