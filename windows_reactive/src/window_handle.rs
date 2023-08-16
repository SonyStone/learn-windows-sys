use windows::{
    core::Result,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{IsWindow, ShowWindow, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE},
    },
};

#[derive(Default, PartialEq, Eq, Clone)]
pub struct WindowHandle(HWND);

impl WindowHandle {
    pub fn new(handle: HWND) -> Self {
        WindowHandle(handle)
    }

    pub fn is_window(&self) -> Result<()> {
        let result = unsafe { IsWindow(self.0) };
        result.ok()
    }

    // + ещё 15 различных состояний.
    pub fn minimize(&self) {
        unsafe {
            ShowWindow(self.0, SW_MINIMIZE);
        }
    }

    pub fn maximize(&self) {
        unsafe {
            ShowWindow(self.0, SW_MAXIMIZE);
        }
    }

    pub fn restore(&self) {
        unsafe {
            ShowWindow(self.0, SW_RESTORE);
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.0
    }
}
