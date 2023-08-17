use windows::Win32::Foundation::HWND;

use crate::{
    user_data_ext::UserDataExt,
    window_handle_ext::{ShowWindowExt, WindowHandleExt},
    window_handle_getter::WindowHandleGetter,
};

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
impl ShowWindowExt for WindowHandle {}
impl UserDataExt for WindowHandle {}
