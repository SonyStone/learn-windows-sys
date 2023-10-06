use windows::{
    core::{ComInterface, Result},
    Win32::{
        Foundation::HWND,
        Graphics::{
            Direct3D::{
                D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP,
                D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0,
                D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2,
                D3D_FEATURE_LEVEL_9_3,
            },
            Direct3D11::{
                D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION,
            },
            Dxgi::{
                Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC},
                IDXGIDevice, IDXGIFactory2, DXGI_ERROR_UNSUPPORTED, DXGI_SWAP_CHAIN_DESC1,
                DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL, DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
    },
};

use super::dxgi_swap_chain::DXGISwapChain;

pub struct Direct3D11Device(ID3D11Device);

impl Direct3D11Device {
    pub fn new() -> Result<Self> {
        let mut result = Direct3D11Device::new_device_with_type(D3D_DRIVER_TYPE_HARDWARE);

        if let Err(err) = &result {
            if err.code() == DXGI_ERROR_UNSUPPORTED {
                result = Direct3D11Device::new_device_with_type(D3D_DRIVER_TYPE_WARP);
            }
        }

        result
    }

    pub fn new_device_with_type(drive_type: D3D_DRIVER_TYPE) -> Result<Self> {
        // This flag adds support for surfaces with a different color channel ordering than the API default.
        // You need it for compatibility with Direct2D.
        let mut flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;

        // This array defines the set of DirectX hardware feature levels this app  supports.
        // The ordering is important and you should  preserve it.
        // Don't forget to declare your app's minimum required feature level in its
        // description.  All apps are assumed to support 9.1 unless otherwise stated.
        let feature_levels = [
            D3D_FEATURE_LEVEL_11_1,
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
            D3D_FEATURE_LEVEL_9_3,
            D3D_FEATURE_LEVEL_9_2,
            D3D_FEATURE_LEVEL_9_1,
        ];

        // optionally set debug and Direct2D compatibility flags
        if cfg!(debug_assertions) {
            flags |= D3D11_CREATE_DEVICE_DEBUG;
        }

        let mut device = None;

        unsafe {
            D3D11CreateDevice(
                None,
                drive_type,
                None,
                flags,
                Some(&feature_levels), // list of feature levels this app can support
                D3D11_SDK_VERSION,
                Some(&mut device), // returns the Direct3D device created
                None,              // returns feature level of device created
                None,              // returns the device immediate context
            )
            .map(|()| device.unwrap())
            .map(Direct3D11Device)
        }
    }

    pub fn create_swapchain(&self, window: HWND) -> Result<DXGISwapChain> {
        let factory = self.get_dxgi_factory()?;

        let props = DXGI_SWAP_CHAIN_DESC1 {
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            ..Default::default()
        };

        unsafe { factory.CreateSwapChainForHwnd(&self.0, window, &props, None, None) }
            .map(DXGISwapChain::new)
    }

    pub fn get_dxgi_factory(&self) -> Result<IDXGIFactory2> {
        unsafe { self.get_dxgi_device()?.GetAdapter()?.GetParent() }
    }

    pub fn get_dxgi_device(&self) -> Result<IDXGIDevice> {
        self.0.cast::<IDXGIDevice>()
    }
}
