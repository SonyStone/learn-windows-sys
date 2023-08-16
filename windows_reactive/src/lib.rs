use hwnd_builder::create_window_handle;
use leptos_reactive::{SignalGet, SignalUpdate};
use message_ext::MessageExt;
use number_into_words::encode;

use window_handle_ext::WindowHandleExt;
use windows::Win32::UI::WindowsAndMessaging::MSG;

mod hwnd_builder;
mod message_ext;
mod param_ext;
mod pcwstr_handler;
mod pre_settings;
mod user_data_ext;
mod window;
mod window_handle;
mod window_handle_ext;

pub fn run() {
    let _ = leptos_reactive::create_scope(leptos_reactive::create_runtime(), |cx| {
        let (count, set_count) = leptos_reactive::create_signal(cx, 0);
        let text = move || {
            let count = count.get();
            match count {
                7 => "Seven is a happy number".to_string(),
                1.. => format!("The number is {}", encode(count as u64)),
                0 => "The number is Zero".to_string(),
                _ => format!("{}", count),
            }
        };

        pre_settings::init_window_class();

        let window = create_window_handle()
            .class_name("NativeWindowsGuiWindow")
            .size((500, 500))
            .position((300, 300))
            .window()
            .resizable()
            .maximizable()
            .minimizable()
            .always_on_top()
            .visible()
            .build();

        println!("Build!");

        let handle = window;

        create_window_handle()
            .class_name("BUTTON")
            .text("maximize")
            .position((10, 60))
            .size((80, 20))
            .parent(&window)
            .push_button()
            .flat()
            .child()
            .visible()
            .on_click(move |_| {
                handle.maximize();
            })
            .build();

        create_window_handle()
            .class_name("BUTTON")
            .text("+1")
            .position((10, 30))
            .size((50, 20))
            .parent(&window)
            .push_button()
            .flat()
            .child()
            .visible()
            .on_click(move |_| {
                set_count.update(|count| *count += 1);
            })
            .build();

        create_window_handle()
            .class_name("BUTTON")
            .text("-1")
            .position((60, 30))
            .size((50, 20))
            .parent(&window)
            .push_button()
            .flat()
            .child()
            .visible()
            .on_click(move |_| {
                set_count.update(|count| *count -= 1);
            })
            .build();

        let text_element = create_window_handle()
            .class_name("STATIC")
            .text("Hello, Windows!")
            .position((10, 10))
            .size((200, 20))
            .parent(&window)
            .child()
            .visible()
            .on_click(move |_| println!("Click? What?"))
            .build();

        leptos_reactive::create_effect(cx, move |_| {
            text_element.set_window_text(&text());
        });

        MSG::dispatch_thread_events();
    });
}
