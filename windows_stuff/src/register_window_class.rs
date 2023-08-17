use windows::{
    core::*,
    Win32::{
        Foundation::HMODULE,
        Graphics::Gdi::{GetStockObject, DKGRAY_BRUSH, HBRUSH},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

use super::wndproc;

pub unsafe fn register_window_class() {
    let instance = GetModuleHandleW(None).unwrap();
    let class_name = w!("Sample Window Class");

    let class = WNDCLASSW {
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        hInstance: instance,
        lpszClassName: class_name,
        hbrBackground: HBRUSH(GetStockObject(DKGRAY_BRUSH).0),
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    };

    // We ignore errors because registering the same window class twice would trigger
    //  an error, and because errors here are detected during CreateWindowEx anyway.
    // Also since there is no weird element in the struct, there is no reason for this
    //  call to fail.
    let atom = RegisterClassW(&class);
    debug_assert!(atom != 0);
}
