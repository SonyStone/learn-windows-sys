use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{HMENU, WINDOW_EX_STYLE, WS_CHILD, WS_VISIBLE},
};

use crate::{window::Window, window_handle_ext::WindowHandleExt};

static TEXT_ID: &HMENU = &HMENU(15);

pub fn create_text(parent: &Window) {
    let button_1 = HWND::new(
        WINDOW_EX_STYLE(0),
        "STATIC",
        "Hello, Windows!",
        WS_CHILD | WS_VISIBLE,
        10,
        10,
        200,
        20,
        parent.handle.hwnd(),
        *TEXT_ID,
    );

    button_1.set_window_text("tetet");
}
