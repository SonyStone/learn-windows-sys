use std::ptr;

use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

struct ComInitialized(*mut ());
impl Drop for ComInitialized {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

thread_local! {
    static COM_INITIALIZED: ComInitialized = {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            ComInitialized(ptr::null_mut())
        }
    };
}

pub fn com_initialized() {
    COM_INITIALIZED.with(|_| {});
}
