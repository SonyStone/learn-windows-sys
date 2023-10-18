use windows::Win32::Graphics::Direct2D::ID2D1Bitmap1;

#[derive(Clone)]
pub struct Bitmap {
    pub(crate) inner: ID2D1Bitmap1,
    pub(crate) empty_image: bool,
}
