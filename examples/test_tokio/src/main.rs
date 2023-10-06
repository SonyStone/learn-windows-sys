use std::{cell::RefCell, rc::Rc};

use glam::IVec2;
use leptos_reactive::{SignalGet, SignalUpdate};
use tokio::sync::mpsc;
use windows_reactive::{
    direct_2d::Direct2d,
    hwnd_builder::create_window_handle,
    message_ext::dispatch_thread_events,
    messages::{message_handler, Message},
    pre_settings,
    window_handle_ext::WindowHandleExt,
    HWND, LPARAM, PAINTSTRUCT, WPARAM,
};

#[derive(Debug)]
struct Win32Message {
    hwnd: HWND,
    msg: u32,
    w: WPARAM,
    l: LPARAM,
}

#[tokio::main]
pub async fn main() {
    let class_name = "NativeWindowsGuiWindow";

    pre_settings::init_window_class(class_name);

    let (tx, mut rx) = mpsc::channel::<Win32Message>(32); // Adjust buffer size as needed

    let mut is_pinter_down = Box::new(false);

    let direct2d = Direct2d::new().unwrap();
    let direct2d = Rc::new(RefCell::new(direct2d));

    let mut last_mouse = Box::new(IVec2::ZERO);

    let window = create_window_handle()
        .class_name(class_name)
        .size((500, 500))
        .position((300, 300))
        .text("Basic Window")
        .window()
        .resizable()
        .maximizable()
        .minimizable()
        .always_on_top()
        .visible()
        .on_message(move |hwnd, msg, w, l| hwnd.default_window_procedure(msg, w, l))
        .build();

    dispatch_thread_events();
}
