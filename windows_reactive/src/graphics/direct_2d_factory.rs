use glam::Vec2;
use windows::{
    core::Result,
    Win32::Graphics::Direct2D::{
        Common::D2D_RECT_F, D2D1CreateFactory, ID2D1Factory1, ID2D1StrokeStyle,
        D2D1_CAP_STYLE_ROUND, D2D1_CAP_STYLE_TRIANGLE, D2D1_DEBUG_LEVEL_INFORMATION,
        D2D1_DEVICE_CONTEXT_OPTIONS_NONE, D2D1_ELLIPSE, D2D1_FACTORY_OPTIONS,
        D2D1_FACTORY_TYPE_MULTI_THREADED, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_ROUNDED_RECT,
        D2D1_STROKE_STYLE_PROPERTIES, D2D1_UNIT_MODE_DIPS,
    },
};

use super::{
    device_context::DeviceContext,
    direct_2d_device::Direct2DDevice,
    direct_3d_device::Direct3D11Device,
    geometry::{EllipseGeometry, PathGeometry, RectangleGeometry, RoundedRectangleGeometry},
};

/// The ID2D1Factory interface is the starting point for using Direct2D;
/// use an ID2D1Factory to create Direct2D resources.
#[derive(Debug)]
pub struct Direct2DFactory(pub ID2D1Factory1);

impl Drop for Direct2DFactory {
    fn drop(&mut self) {
        println!("🚮 Direct2DFactory dropped here")
    }
}

impl Direct2DFactory {
    /// Create a new Direct2D factory.
    ///
    /// This requires Windows 7 platform update, and can also fail if
    /// resources are unavailable.
    pub fn new() -> Result<Self> {
        let mut options = D2D1_FACTORY_OPTIONS::default();

        if cfg!(debug_assertions) {
            options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
        }

        unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_MULTI_THREADED, Some(&options)) }
            .map(Direct2DFactory)
    }

    pub fn create_device(&self, device: &Direct3D11Device) -> Result<Direct2DDevice> {
        // Obtain the Direct2D device for 2-D rendering.
        let device = unsafe { self.0.CreateDevice(&device.get_dxgi_device()?)? };

        let device = Direct2DDevice(device);

        Ok(device)
    }

    pub fn get_desktop_dpi(&self) -> f32 {
        let mut dpi = 0.0;
        unsafe { self.0.GetDesktopDpi(&mut dpi, &mut dpi) };
        dpi / 96.0
    }

    pub fn create_path_geometry(&self) -> Result<PathGeometry> {
        let geometry = unsafe { self.0.CreatePathGeometry()? };
        Ok(PathGeometry(geometry))
    }

    pub fn create_rect_geometry(&self, rect: &D2D_RECT_F) -> Result<RectangleGeometry> {
        let geometry = unsafe { self.0.CreateRectangleGeometry(rect)? };
        Ok(RectangleGeometry(geometry))
    }

    pub fn create_round_rect_geometry(
        &self,
        rect: &D2D1_ROUNDED_RECT,
    ) -> Result<RoundedRectangleGeometry> {
        let geometry = unsafe { self.0.CreateRoundedRectangleGeometry(rect)? };
        Ok(RoundedRectangleGeometry(geometry))
    }

    pub fn create_circle_geometry(&self, circle: &D2D1_ELLIPSE) -> Result<EllipseGeometry> {
        let geometry = unsafe { self.0.CreateEllipseGeometry(circle)? };
        Ok(EllipseGeometry(geometry))
    }

    /// [ID2D1StrokeStyle](https://learn.microsoft.com/en-us/windows/win32/api/d2d1/nn-d2d1-id2d1strokestyle)
    ///
    // TODO
    pub fn create_stroke_style(
        &self,
        // props: &D2D1_STROKE_STYLE_PROPERTIES,
        dashes: Option<&[f32]>,
    ) -> Result<ID2D1StrokeStyle> {
        let props: D2D1_STROKE_STYLE_PROPERTIES = D2D1_STROKE_STYLE_PROPERTIES {
            startCap: D2D1_CAP_STYLE_ROUND,
            endCap: D2D1_CAP_STYLE_TRIANGLE,
            ..Default::default()
        };

        unsafe { self.0.CreateStrokeStyle(&props, dashes) }
    }
}
