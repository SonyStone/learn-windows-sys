use windows::Win32::Graphics::Direct2D::{
    ID2D1EllipseGeometry, ID2D1Geometry, ID2D1PathGeometry, ID2D1RectangleGeometry,
    ID2D1RoundedRectangleGeometry,
};

pub struct PathGeometry(pub(crate) ID2D1PathGeometry);

pub struct RectangleGeometry(pub(crate) ID2D1RectangleGeometry);

pub struct RoundedRectangleGeometry(pub(crate) ID2D1RoundedRectangleGeometry);

pub struct EllipseGeometry(pub(crate) ID2D1EllipseGeometry);

pub struct Geometry(pub(crate) ID2D1Geometry);

// impl From<PathGeometry> for Geometry {
//     fn from(pg: PathGeometry) -> Self {
//         Geometry(pg.0.)
//     }
// }

// impl From<RectangleGeometry> for Geometry {
//     fn from(pg: RectangleGeometry) -> Self {
//         Geometry(pg.0.up())
//     }
// }

// impl From<RoundedRectangleGeometry> for Geometry {
//     fn from(pg: RoundedRectangleGeometry) -> Self {
//         Geometry(pg.0.up())
//     }
// }

// impl From<EllipseGeometry> for Geometry {
//     fn from(pg: EllipseGeometry) -> Self {
//         Geometry(pg.0.up())
//     }
// }
