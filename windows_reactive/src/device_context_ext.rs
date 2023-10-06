use glam::IVec2;
use windows::Win32::{
    Foundation::{COLORREF, HWND},
    Graphics::Gdi::{Ellipse, GetDC, ReleaseDC, SetPixel, HDC},
};

pub trait DeviceContextGetter {
    fn get_device_context(&self) -> &HDC;
}

impl DeviceContextGetter for HDC {
    fn get_device_context(&self) -> &HDC {
        self
    }
}

impl DeviceContextExt for HDC {}

pub trait DeviceContextExt: DeviceContextGetter {
    fn get_device_context(hwnd: &HWND) -> HDC {
        unsafe { GetDC(*hwnd) }
    }
    fn set_pixel(&self, position: IVec2, color: COLORREF) {
        unsafe { SetPixel(*self.get_device_context(), position.x, position.y, color) };
    }
    fn ellipse(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { Ellipse(*self.get_device_context(), left, top, right, bottom) };
    }
    fn release_device_context(&self, hwnd: &HWND) {
        unsafe { ReleaseDC(*hwnd, *self.get_device_context()) };
    }
}
