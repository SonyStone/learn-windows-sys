use std::{ffi::OsString, os::windows::prelude::OsStringExt};

use windows::{
    w,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            CreateWindowExW, GetClassNameW, MessageBoxW, BN_CLICKED, BN_DBLCLK, BN_PUSHED, BS_FLAT,
            BS_PUSHBUTTON, EN_CHANGE, HMENU, MB_ICONINFORMATION, MB_OK, WINDOW_EX_STYLE,
            WINDOW_STYLE, WS_CHILD, WS_VISIBLE,
        },
    },
};

use crate::{
    param_ext::ParamExt,
    window::Window,
    window_handle::{self},
    window_handle_ext::{self, WindowHandleExt},
};

static BUTTON_ID: &HMENU = &HMENU(15);

pub fn create_button(parent: Window) -> () {
    let button_1 = HWND::new(
        WINDOW_EX_STYLE(0),
        "BUTTON",
        "OK",
        WINDOW_STYLE(BS_PUSHBUTTON as u32) | WINDOW_STYLE(BS_FLAT as u32) | WS_CHILD | WS_VISIBLE,
        // WS_TABSTOP
        //     | WS_VISIBLE
        //     | WINDOW_STYLE(BS_NOTIFY as u32)
        //     | WS_CHILD
        //     | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
        10,
        10,
        100,
        100,
        parent.handle.hwnd(),
        *BUTTON_ID,
    );
}

pub fn on_button_click(hwnd: HWND) {
    unsafe {
        MessageBoxW(
            hwnd,
            w!("You clicked the button!"),
            w!("Clicked"),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

pub fn button_handler(hwnd: HWND, w: WPARAM, l: LPARAM) -> bool {
    let child_handle: HWND = HWND(l.0);
    let message = w.get_hiword();

    let class_name = unsafe { get_class_name(child_handle) };

    match &class_name as &str {
        "Button" => match message {
            BN_CLICKED => {
                println!("Button BN_CLICKED");
                hwnd.move_window(0, 0, 400, 400, true);
                // on_button_click(hwnd);
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

unsafe fn get_class_name(handle: HWND) -> String {
    let mut class_name_raw: [u16; 100] = [0; 100];
    let count = GetClassNameW(handle, &mut class_name_raw) as usize;
    let class_name = &class_name_raw[..count];
    let class_name = OsString::from_wide(class_name);
    class_name.into_string().unwrap_or("".to_string())
}
