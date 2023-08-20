use std::mem::size_of;

use windows::Win32::{
    Foundation::{GetLastError, ERROR_CLASS_ALREADY_EXISTS, HWND, LPARAM, LRESULT, WPARAM},
    Graphics::Gdi::{GetStockObject, DKGRAY_BRUSH, HBRUSH},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        LoadCursorW, RegisterClassExW, CS_DBLCLKS, CS_HREDRAW, CS_VREDRAW, IDC_ARROW, WM_NCCREATE,
        WNDCLASSEXW, WNDCLASS_STYLES,
    },
};

use crate::{
    hwnd_builder::Callback,
    param_ext::LParamExt,
    pcwstr_handler::{AsPCWSTR, AsWide},
    window_handle_ext::WindowHandleExt,
};

/// Create the window class for the base nwg window
pub fn init_window_class(class_name: &str) {
    unsafe {
        let hmod = GetModuleHandleW(None).unwrap();
        if hmod.is_invalid() {
            panic!("GetModuleHandleW failed")
        }

        let class_name = class_name.as_wide();

        let background = HBRUSH(GetStockObject(DKGRAY_BRUSH).0);
        // let style = CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS;
        let style = WNDCLASS_STYLES::default();

        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: hmod,
            lpszClassName: class_name.as_pcwstr(),
            hbrBackground: background,
            style,
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        };

        let class_token = RegisterClassExW(&class);
        if class_token == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS {
            panic!("System class creation failed")
        }
    }
}

/**
    A blank system procedure used when creating new window class. Actual system event handling is done in the subclass procedure `process_events`.
*/
unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    match msg {
        WM_NCCREATE => {
            hwnd.set_user_data(l.get_create_struct().lpCreateParams);
            hwnd.default_window_proc(msg, w, l)
        }
        _ => hwnd.get_user_data::<Callback>().map_or_else(
            || hwnd.default_window_proc(msg, w, l),
            |callback| callback.0(hwnd, msg, w, l),
        ),
    }
}
