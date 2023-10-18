use windows::Win32::Graphics::Direct2D::{ID2D1Brush, ID2D1SolidColorBrush};

#[derive(Clone, Debug)]
pub struct Brush(pub(crate) ID2D1Brush);

#[derive(Clone, Debug)]
pub struct SolidColorBrush(pub(crate) ID2D1SolidColorBrush);
