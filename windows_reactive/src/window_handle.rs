use windows::Win32::Foundation::HWND;

use crate::{window_handle_ext::WindowHandleExt, window_handle_getter::WindowHandleGetter};

#[derive(Default, PartialEq, Eq, Clone)]
pub struct WindowHandle(HWND);

impl WindowHandle {
    pub fn new(handle: HWND) -> Self {
        WindowHandle(handle)
    }
}

impl WindowHandleGetter for WindowHandle {
    fn get_handle(&self) -> &HWND {
        &self.0
    }
}

impl WindowHandleExt for WindowHandle {}
