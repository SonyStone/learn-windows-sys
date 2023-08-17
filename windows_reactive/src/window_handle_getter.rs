use windows::Win32::Foundation::HWND;

pub trait WindowHandleGetter {
    fn get_handle(&self) -> &HWND;
}

impl WindowHandleGetter for HWND {
    fn get_handle(&self) -> &HWND {
        self
    }
}
