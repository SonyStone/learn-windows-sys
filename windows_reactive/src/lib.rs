use message_ext::MessageExt;
use windows::Win32::UI::WindowsAndMessaging::MSG;

mod button;
mod message_ext;
mod param_ext;
mod pcwstr_handler;
mod pre_settings;
mod user_data_ext;
mod window;
mod window_handle;
mod window_handle_ext;

pub fn run() {
    let window = window::window().build();

    button::create_button(window);

    dispatch_thread_events();
}

/**
    Dispatch system events in the current thread. This method will pause the thread until there are events to process.
*/
pub fn dispatch_thread_events() {
    let mut message: MSG = MSG::default();

    while message.get_message().into() {
        if !message.is_dialog_message() {
            message.translate_message();
            message.dispatch_message();
        }
    }
}
