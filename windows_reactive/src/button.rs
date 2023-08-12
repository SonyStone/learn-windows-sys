use windows::{
    w,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            MessageBoxW, BN_CLICKED, BN_DBLCLK, BN_PUSHED, BS_FLAT, BS_PUSHBUTTON, EN_CHANGE,
            HMENU, MB_ICONINFORMATION, MB_OK, WINDOW_EX_STYLE, WINDOW_STYLE, WS_CHILD, WS_VISIBLE,
        },
    },
};

use crate::{param_ext::ParamExt, window::Window, window_handle_ext::WindowHandleExt};

static BUTTON_ID: &HMENU = &HMENU(15);

pub fn create_button(parent: &Window) -> () {
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

    let class_name = child_handle.get_class_name();

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
