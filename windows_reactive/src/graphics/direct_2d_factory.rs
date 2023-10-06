use glam::Vec2;
use windows::{
    core::Result,
    Win32::Graphics::Direct2D::{
        D2D1CreateFactory, ID2D1Factory1, ID2D1StrokeStyle, D2D1_CAP_STYLE_ROUND,
        D2D1_CAP_STYLE_TRIANGLE, D2D1_DEBUG_LEVEL_INFORMATION, D2D1_DEVICE_CONTEXT_OPTIONS_NONE,
        D2D1_FACTORY_OPTIONS, D2D1_FACTORY_TYPE_MULTI_THREADED, D2D1_FACTORY_TYPE_SINGLE_THREADED,
        D2D1_STROKE_STYLE_PROPERTIES, D2D1_UNIT_MODE_DIPS,
    },
};

use super::{direct_2d_device_context::Direct2DDeviceContext, direct_3d_device::Direct3D11Device};

/// The ID2D1Factory interface is the starting point for using Direct2D;
/// use an ID2D1Factory to create Direct2D resources.
pub struct Direct2DFactory(ID2D1Factory1);

impl Direct2DFactory {
    pub fn create_factory() -> Result<Self> {
        let mut options = D2D1_FACTORY_OPTIONS::default();

        if cfg!(debug_assertions) {
            options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
        }

        unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_MULTI_THREADED, Some(&options)) }
            .map(Direct2DFactory)
    }

    /// [ID2D1StrokeStyle](https://learn.microsoft.com/en-us/windows/win32/api/d2d1/nn-d2d1-id2d1strokestyle)
    ///
    pub fn create_stroke_style(&self) -> Result<ID2D1StrokeStyle> {
        let props: D2D1_STROKE_STYLE_PROPERTIES = D2D1_STROKE_STYLE_PROPERTIES {
            startCap: D2D1_CAP_STYLE_ROUND,
            endCap: D2D1_CAP_STYLE_TRIANGLE,
            ..Default::default()
        };

        unsafe { self.0.CreateStrokeStyle(&props, None) }
    }

    pub fn get_desktop_dpi(&self) -> Vec2 {
        let mut dpi = Vec2::ZERO;
        unsafe { self.0.GetDesktopDpi(&mut dpi.x, &mut dpi.y) };
        dpi
    }

    /// creates [ID2D1DeviceContext](https://learn.microsoft.com/en-us/windows/win32/api/d2d1_1/nn-d2d1_1-id2d1devicecontext)
    ///
    /// Represents a set of state and command buffers that are used to render to a target.
    pub fn create_render_target(&self, device: &Direct3D11Device) -> Result<Direct2DDeviceContext> {
        unsafe {
            // Obtain the Direct2D device for 2-D rendering.
            let d2device = self.0.CreateDevice(&device.get_dxgi_device()?)?;

            // Get Direct2D device's corresponding device context object.
            let target = Direct2DDeviceContext(
                d2device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?,
            );

            target.set_unit_mode(D2D1_UNIT_MODE_DIPS);

            Ok(target)
        }
    }
}
