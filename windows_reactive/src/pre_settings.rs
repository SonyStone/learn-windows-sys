use std::mem::size_of;

use windows::{
    w,
    Win32::{
        Foundation::{GetLastError, ERROR_CLASS_ALREADY_EXISTS, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{GetStockObject, DKGRAY_BRUSH, HBRUSH},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            DefWindowProcW, LoadCursorW, PostMessageW, RegisterClassExW, ShowWindow, CS_HREDRAW,
            CS_VREDRAW, IDC_ARROW, SW_HIDE, WM_CLOSE, WM_COMMAND, WM_CREATE, WM_LBUTTONDOWN,
            WM_RBUTTONDOWN, WM_USER, WNDCLASSEXW,
        },
    },
};

use crate::{
    button,
    user_data_ext::{self, Callback, UserDataExt},
    window_handle_ext::WindowHandleExt,
};

/// Create the window class for the base nwg window
pub(crate) fn init_window_class() -> () {
    unsafe {
        /**
            A blank system procedure used when creating new window class. Actual system event handling is done in the subclass procedure `process_events`.
        */
        unsafe extern "system" fn blank_window_proc(
            hwnd: HWND,
            msg: u32,
            w: WPARAM,
            l: LPARAM,
        ) -> LRESULT {
            let handled = match msg {
                WM_CREATE => {
                    println!("WM_CREATE");
                    // let my_data = l.0 as *mut String;
                    // let my_data = Box::from_raw(my_data);
                    // println!("Data in WM_CREATE: {}", my_data);
                    // hwnd.add_raw(Callback::from_raw(l.0));

                    PostMessageW(hwnd, WM_USER + 101, WPARAM(0), LPARAM(0));
                    true
                }
                WM_CLOSE => {
                    println!("WM_CLOSE");
                    ShowWindow(hwnd, SW_HIDE);
                    true
                }
                WM_LBUTTONDOWN => {
                    println!("WM_LBUTTONDOWN");
                    // let callback = hwnd.get().unwrap();
                    // callback.call();
                    true
                }
                WM_RBUTTONDOWN => {
                    println!("WM_RBUTTONDOWN");
                    hwnd.set_window_text("Hello world!");
                    true
                }
                WM_COMMAND => button::button_handler(hwnd, w, l),
                _ => false,
            };

            if handled {
                LRESULT(0)
            } else {
                DefWindowProcW(hwnd, msg, w, l)
            }
        }

        let hmod = GetModuleHandleW(None).unwrap();
        if hmod.is_invalid() {
            panic!("GetModuleHandleW failed")
        }

        let class_name = w!("NativeWindowsGuiWindow");
        let background = HBRUSH(GetStockObject(DKGRAY_BRUSH).0);
        let style = CS_HREDRAW | CS_VREDRAW;

        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: hmod,
            lpszClassName: class_name,
            hbrBackground: background,
            style,
            lpfnWndProc: Some(blank_window_proc),
            ..Default::default()
        };

        let class_token = RegisterClassExW(&class);
        if class_token == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS {
            panic!("System class creation failed")
        } else {
            ()
        }
    }

    ()
}
