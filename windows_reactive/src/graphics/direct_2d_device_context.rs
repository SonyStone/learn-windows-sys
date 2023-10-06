use windows::{
    core::{IntoParam, Result},
    Foundation::Numerics::Matrix3x2,
    Win32::Graphics::{
        Direct2D::{
            Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_F},
            ID2D1Bitmap1, ID2D1Brush, ID2D1DeviceContext, ID2D1Geometry, ID2D1Image,
            ID2D1SolidColorBrush, ID2D1StrokeStyle, D2D1_BITMAP_PROPERTIES1, D2D1_BRUSH_PROPERTIES,
            D2D1_ELLIPSE, D2D1_UNIT_MODE,
        },
        Dxgi::IDXGISurface,
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
pub struct Direct2DDeviceContext(pub(crate) ID2D1DeviceContext);

impl Direct2DDeviceContext {
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
    pub fn create_bitmap_from_dxgi_surface<P0>(
        &self,
        surface: P0,
        props: Option<*const D2D1_BITMAP_PROPERTIES1>,
    ) -> Result<ID2D1Bitmap1>
    where
        P0: IntoParam<IDXGISurface>,
    {
        unsafe { self.0.CreateBitmapFromDxgiSurface(surface, props) }
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
    pub fn set_dpi(&self, dpi: Vec2) {
        unsafe { self.0.SetDpi(dpi.x, dpi.y) };
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

    pub fn draw_geometry<P0, P1, P2>(
        &self,
        geometry: P0,
        brush: P1,
        strokewidth: f32,
        strokestyle: P2,
    ) where
        P0: IntoParam<ID2D1Geometry>,
        P1: IntoParam<ID2D1Brush>,
        P2: IntoParam<ID2D1StrokeStyle>,
    {
        unsafe {
            self.0
                .DrawGeometry(geometry, brush, strokewidth, strokestyle)
        }
    }

    pub fn draw_line<P0, P1>(
        &self,
        point: (Vec2, Vec2),
        brush: P0,
        strokewidth: f32,
        strokestyle: P1,
    ) where
        P0: IntoParam<ID2D1Brush>,
        P1: IntoParam<ID2D1StrokeStyle>,
    {
        let point0 = D2D_POINT_2F {
            x: point.0.x,
            y: point.0.y,
        };
        let point1 = D2D_POINT_2F {
            x: point.1.x,
            y: point.1.y,
        };
        unsafe {
            self.0
                .DrawLine(point0, point1, brush, strokewidth, strokestyle)
        }
    }

    pub fn set_transform(&self, transform: &Affine2) {
        let mat = to_matrix3x2(transform);
        unsafe { self.0.SetTransform(&mat) }
    }
}

/// Part of `ID2D1RenderTarget` interface.
impl Direct2DDeviceContext {
    /// Initiates drawing on this render target.
    pub fn begin_draw(&self) {
        unsafe { self.0.BeginDraw() };
    }

    /// Clears the drawing area to the specified color.
    pub fn clear(&self, clearcolor: Option<*const D2D1_COLOR_F>) {
        unsafe { self.0.Clear(clearcolor) };
    }

    pub fn draw_ellipse<P0, P1>(
        &self,
        ellipse: *const D2D1_ELLIPSE,
        brush: P0,
        strokewidth: f32,
        strokestyle: P1,
    ) where
        P0: IntoParam<ID2D1Brush>,
        P1: IntoParam<ID2D1StrokeStyle>,
    {
        unsafe {
            self.0.DrawEllipse(ellipse, brush, strokewidth, strokestyle);
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

    pub fn end_draw(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> Result<()> {
        unsafe { self.0.EndDraw(tag1, tag2) }
    }
}

use glam::{Affine2, Vec2};

fn to_matrix3x2(value: &Affine2) -> Matrix3x2 {
    Matrix3x2 {
        M11: value.matrix2.x_axis.x,
        M12: value.matrix2.x_axis.y,
        M21: value.matrix2.y_axis.x,
        M22: value.matrix2.y_axis.y,
        M31: value.translation.x,
        M32: value.translation.y,
    }
}
