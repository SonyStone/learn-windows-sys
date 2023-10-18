use glam::{Affine2, Vec2};
use windows::{
    core::{IntoParam, Result},
    Foundation::Numerics::Matrix3x2,
    Win32::Graphics::{
        Direct2D::{
            Common::{
                D2D1_ALPHA_MODE_IGNORE, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_RECT_F, D2D_SIZE_F,
            },
            ID2D1Brush, ID2D1DeviceContext, ID2D1Image, ID2D1SolidColorBrush, ID2D1StrokeStyle,
            D2D1_BITMAP_OPTIONS_CANNOT_DRAW, D2D1_BITMAP_OPTIONS_TARGET, D2D1_BITMAP_PROPERTIES1,
            D2D1_BRUSH_PROPERTIES, D2D1_ELLIPSE, D2D1_UNIT_MODE,
        },
        Dxgi::{Common::DXGI_FORMAT_B8G8R8A8_UNORM, IDXGISurface},
    },
};

/// [ID2D1DeviceContext](https://learn.microsoft.com/en-us/windows/win32/api/d2d1_1/nn-d2d1_1-id2d1devicecontext)
///
/// Represents a set of state and command buffers that are used to render to a target.
/// The device context can render to a target bitmap or a command list.
///
/// Part of Direct2D
///
/// `ID2D1DeviceContext` is an interface in the Direct2D API that represents a drawing context
/// for 2D graphics rendering on a specific device. It provides methods and functionality for
/// drawing shapes, text, images, and performing various 2D graphics operations. This interface
/// is a fundamental part of Direct2D and is used to issue drawing commands to the graphics
/// hardware.
///
/// Here are some key aspects and purposes of ID2D1DeviceContext:
/// - __Graphics Device__ Independence: `ID2D1DeviceContext` abstracts the underlying graphics
/// hardware, allowing your application to render graphics without worrying about the specific
/// graphics hardware details. It provides a unified interface for rendering on various devices,
/// including both software and hardware rendering.
/// - __Drawing Primitives__: You can use `ID2D1DeviceContext` to draw a wide range of 2D
/// graphics primitives, including lines, rectangles, ellipses, paths, and custom shapes. It
/// supports both filled and outlined (stroked) versions of these primitives.
/// - __Text Rendering__: `ID2D1DeviceContext` allows you to render text using TrueType fonts or
/// other font formats supported by DirectWrite. You can control various text rendering
/// attributes such as font size, style, and alignment.
/// - __Bitmap Rendering__: You can create and render bitmaps with ID2D1DeviceContext. This is
/// useful for rendering images or textures onto your graphics surface.
/// - __Layering__: Direct2D supports layering, allowing you to create and manage layers within a
/// `ID2D1DeviceContext`. Layers are useful for isolating groups of drawing commands and applying effects or transformations to them.
/// - __Transformations__: `ID2D1DeviceContext` allows you to apply transformations to graphics
/// operations, including translation, rotation, scaling, and skewing. You can transform the
/// entire context or individual drawing commands.
/// - __Antialiasing and Quality Settings__: You can control the antialiasing and rendering quality
/// settings to achieve the desired visual appearance for your graphics. This includes
/// specifying the level of antialiasing and rendering modes.
/// - __Resource Management__: `ID2D1DeviceContext` manages resources like brushes, bitmaps,
/// and geometries. These resources can be created and reused to optimize rendering performance.
#[derive(Debug)]
pub struct DeviceContext(pub(crate) ID2D1DeviceContext);

impl Drop for DeviceContext {
    fn drop(&mut self) {
        println!("🚮 DeviceContext dropped here")
    }
}

impl DeviceContext {
    /// Create a new device context from an existing COM object.
    ///
    /// Marked as unsafe because the device must be in a good state.
    /// This *might* be overly conservative.
    pub unsafe fn new(device_context: ID2D1DeviceContext) -> DeviceContext {
        DeviceContext(device_context)
    }

    /// Creates a bitmap that can be used as a target surface, for reading back to the CPU, or as a source for the
    /// DrawBitmap and ID2D1BitmapBrush APIs. In addition, color context information can be passed to the bitmap.
    pub fn create_bitmap() {
        todo!()
    }

    pub fn create_bitmap_brush() {
        todo!()
    }

    /// [CreateBitmapFromDxgiSurface](https://learn.microsoft.com/en-us/windows/win32/api/d2d1_1/nf-d2d1_1-id2d1devicecontext-createbitmapfromdxgisurface(idxgisurface_constd2d1_bitmap_properties1__id2d1bitmap1))
    ///
    /// Creates a bitmap from a DXGI surface that can be set as a target surface or have additional color context information specified.
    ///
    /// Most often, this bitmap will be used to set the target of a
    /// DeviceContext.
    ///
    /// Assumes RGBA8 format and premultiplied alpha.
    ///
    /// The `unsafe` might be conservative, but we assume the `dxgi`
    /// argument is in good shape to be a target.
    pub fn create_bitmap_from_dxgi<P0>(&self, dxgi: P0, dpi_scale: f32) -> Result<Bitmap>
    where
        P0: IntoParam<IDXGISurface>,
    {
        let props = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_IGNORE,
            },
            dpiX: 96.0 * dpi_scale,
            dpiY: 96.0 * dpi_scale,
            bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
            ..Default::default()
        };
        let bitmap = unsafe { self.0.CreateBitmapFromDxgiSurface(dxgi, Some(&props))? };

        Ok(Bitmap {
            inner: bitmap,
            empty_image: false,
        })
    }

    pub fn create_bitmap_from_wic_bitmap() {
        todo!()
    }

    pub fn create_color_context() {
        todo!()
    }

    // CreateColorContextFromFilename

    // CreateColorContextFromWicColorContext

    // CreateCommandList

    // CreateGradientStopCollection

    /// [SetDpi](https://learn.microsoft.com/en-us/windows/win32/api/d2d1/nf-d2d1-id2d1rendertarget-setdpi)
    ///
    /// allows you to set the dots per inch (DPI) settings for the rendering context. DPI is a measure
    /// of the resolution of a display or output device, and it affects how graphics and text are scaled
    /// and rendered. By setting the DPI, you can control the scaling behavior of your graphics to
    /// ensure they appear correctly on different display devices with varying DPI values.
    pub fn set_dpi_scale(&self, dpi_scale: f32) {
        unsafe { self.0.SetDpi(96. * dpi_scale, 96. * dpi_scale) };
    }

    pub fn get_dpi_scale(&self) -> Vec2 {
        let mut dpi = Vec2::ZERO;
        unsafe { self.0.GetDpi(&mut dpi.x, &mut dpi.y) };
        // https://docs.microsoft.com/en-us/windows/win32/direct2d/direct2d-and-high-dpi
        dpi / 96.0
    }

    /// [SetTarget](https://learn.microsoft.com/en-us/windows/win32/api/d2d1_1/nf-d2d1_1-id2d1devicecontext-settarget)
    ///
    /// The bitmap or command list to which the Direct2D device context will now render.
    pub fn set_target<P0>(&self, bitmap: P0)
    where
        P0: IntoParam<ID2D1Image>,
    {
        unsafe { self.0.SetTarget(bitmap) };
    }

    pub fn get_size(&self) -> D2D_SIZE_F {
        unsafe { self.0.GetSize() }
    }

    pub fn create_solid_color_brush(
        &self,
        color: *const D2D1_COLOR_F,
        brush_properties: Option<*const D2D1_BRUSH_PROPERTIES>,
    ) -> Result<ID2D1SolidColorBrush> {
        unsafe { self.0.CreateSolidColorBrush(color, brush_properties) }
    }

    pub fn set_unit_mode(&self, unitmode: D2D1_UNIT_MODE) {
        unsafe { self.0.SetUnitMode(unitmode) };
    }
}

/// Part of `ID2D1RenderTarget` interface.
///
/// Draw stuff goes here
impl DeviceContext {
    /// Begin drawing.
    ///
    /// Initiates drawing on this render target.
    ///
    /// This must be done before any piet drawing operations.
    ///
    /// There may be safety concerns (not clear what happens if the sequence
    /// is not followed).
    pub fn begin_draw(&self) {
        unsafe { self.0.BeginDraw() };
    }

    /// End drawing.
    pub fn end_draw(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> Result<()> {
        unsafe { self.0.EndDraw(tag1, tag2) }
    }

    /// Clears the drawing area to the specified color.
    pub fn clear(&self, clearcolor: Option<*const D2D1_COLOR_F>) {
        unsafe { self.0.Clear(clearcolor) };
    }

    pub fn set_transform(&self, transform: &Affine2) {
        let mat = affine_to_matrix3x2(transform);
        unsafe { self.0.SetTransform(&mat) }
    }

    pub(crate) fn set_transform_identity(&mut self) {
        let mat = affine_to_matrix3x2(&Affine2::IDENTITY);
        unsafe { self.0.SetTransform(&mat) }
    }

    pub(crate) fn get_transform(&mut self) -> Affine2 {
        let mut transform = Matrix3x2::identity();
        unsafe { self.0.GetTransform(&mut transform) }
        matrix3x2_to_affine(&transform)
    }

    pub(crate) fn fill_geometry(
        &mut self,
        geometry: &Geometry,
        brush: &Brush,
        opacity_brush: Option<&Brush>,
    ) {
        unsafe {
            self.0
                .FillGeometry(&geometry.0, &brush.0, opacity_brush.map(|f| &f.0))
        };
    }

    pub fn draw_geometry<P2>(
        &self,
        geometry: &Geometry,
        brush: &Brush,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        unsafe {
            self.0.DrawGeometry(
                &geometry.0,
                &brush.0,
                stroke_width,
                stroke_style.map(|f| &f.0),
            )
        }
    }

    pub fn draw_line<P0>(
        &self,
        point: (Vec2, Vec2),
        brush: P0,
        strokewidth: f32,
        strokestyle: Option<&StrokeStyle>,
    ) where
        P0: IntoParam<ID2D1Brush>,
    {
        unsafe {
            self.0.DrawLine(
                vec2_to_point_2f(point.0),
                vec2_to_point_2f(point.1),
                brush,
                strokewidth,
                strokestyle.map(|f| &f.0),
            )
        }
    }

    pub fn draw_ellipse<P0>(
        &self,
        ellipse: *const D2D1_ELLIPSE,
        brush: P0,
        strokewidth: f32,
        strokestyle: Option<&StrokeStyle>,
    ) where
        P0: IntoParam<ID2D1Brush>,
    {
        unsafe {
            self.0
                .DrawEllipse(ellipse, brush, strokewidth, strokestyle.map(|f| &f.0));
        };
    }

    /// [DrawRectangle](https://learn.microsoft.com/en-us/windows/win32/api/d2d1/nf-d2d1-id2d1rendertarget-drawrectangle(constd2d1_rect_f__id2d1brush_float_id2d1strokestyle))
    pub fn draw_rectangle<P0, P1>(
        &self,
        rect: *const D2D_RECT_F,
        brush: P0,
        strokewidth: f32,
        strokestyle: P1,
    ) where
        P0: IntoParam<ID2D1Brush>,
        P1: IntoParam<ID2D1StrokeStyle>,
    {
        unsafe {
            self.0.DrawRectangle(rect, brush, strokewidth, strokestyle);
        };
    }
}

use super::{
    bitmap::Bitmap,
    brush::Brush,
    conv::{affine_to_matrix3x2, matrix3x2_to_affine, vec2_to_point_2f},
    geometry::Geometry,
    stroke_style::StrokeStyle,
};
