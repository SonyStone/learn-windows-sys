use std::mem::size_of;

use windows::{
    w,
    Win32::{
        Foundation::{GetLastError, ERROR_CLASS_ALREADY_EXISTS, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{GetStockObject, DKGRAY_BRUSH, HBRUSH},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            DefWindowProcW, LoadCursorW, PostMessageW, PostQuitMessage, RegisterClassExW,
            ShowWindow, BN_CLICKED, BN_DBLCLK, BN_PUSHED, CS_HREDRAW, CS_VREDRAW, EN_CHANGE,
            IDC_ARROW, SW_HIDE, WM_CLOSE, WM_COMMAND, WM_CREATE, WM_DESTROY, WM_LBUTTONDOWN,
            WM_RBUTTONDOWN, WM_USER, WNDCLASSEXW,
        },
    },
};

use crate::{
    param_ext::{LParamExt, ParamExt},
    pcwstr_handler::{AsPCWSTR, AsWide},
    user_data_ext::UserDataExt,
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
        let style = CS_HREDRAW | CS_VREDRAW;

        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: hmod,
            lpszClassName: class_name.as_pcwstr(),
            hbrBackground: background,
            style,
            lpfnWndProc: Some(blank_window_proc),
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
unsafe extern "system" fn blank_window_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    let handled = match msg {
        WM_CREATE => {
            println!("WM_CREATE");
            let text = l.get_any::<&str>();
            println!("Data in WM_CREATE: {}", text);

            // hwnd.add_raw(Callback::from_raw(l.0));

            PostMessageW(hwnd, WM_USER + 101, WPARAM(0), LPARAM(0));
            true
        }
        WM_CLOSE => {
            println!("WM_CLOSE");
            // ShowWindow(hwnd, SW_HIDE);
            false
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            PostQuitMessage(0);
            false
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
        WM_COMMAND => {
            let child_handle: HWND = HWND(l.0);
            let message = w.get_hiword();
            let id = w.get_loword();

            let class_name = child_handle.get_class_name();
            println!("WM_COMMAND class_name: {}", class_name);

            match &class_name as &str {
                "Button" => match message {
                    BN_CLICKED => {
                        println!("Button BN_CLICKED {}", id);
                        child_handle.get().unwrap().call(child_handle);
                    }
                    BN_DBLCLK => {
                        println!("Button BN_DBLCLK")
                    }
                    BN_PUSHED => {
                        println!("Button BN_PUSHED")
                    }
                    _ => {
                        println!("Button WUT?")
                    }
                },
                "Edit" => match message {
                    EN_CHANGE => {
                        println!("Edit EN_CHANGE")
                    }
                    _ => {
                        println!("Edit WUT?")
                    }
                },
                _ => {
                    println!("WUT? WUT?")
                }
            };

            true
        }
        _ => false,
    };

    if handled {
        LRESULT(0)
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}
