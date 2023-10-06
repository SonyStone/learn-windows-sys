use glam::IVec2;
use leptos_reactive::{SignalGet, SignalUpdate};
use number_into_words::encode;
use windows_reactive::{
    device_context_ext::DeviceContextExt,
    hwnd_builder::{create_window_handle, Callback},
    message_ext::dispatch_thread_events,
    messages::{message_handler, Command, Message, PointerType},
    pre_settings,
    window_handle_ext::WindowHandleExt,
    PostQuitMessage, COLORREF, HDC, HWND, PAINTSTRUCT,
};

pub fn main() {
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

        let class_name = "NativeWindowsGuiWindow";

        pre_settings::init_window_class(class_name);

        let mut index = 0;

        let on_click = |hwnd: HWND| hwnd.set_window_text("Basic Window");
        let on_right_click = |hwnd: HWND| hwnd.set_window_text("Hello world!");

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
            .on_message(move |hwnd, msg, w, l| {
                let message = message_handler(hwnd, msg, w, l);
                match message {
                    Message::Create(_) => {
                        println!("WM_CREATE");
                        hwnd.handled()
                    }
                    Message::Close => {
                        println!("WM_CLOSE");
                        hwnd.hide();
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    Message::Paint => {
                        index += 1;
                        println!("WM_PAINT {}", index);

                        let mut ps = PAINTSTRUCT::default();
                        let r = hwnd.get_client_rect();

                        if r.bottom == 0 {
                            hwnd.default_window_procedure(msg, w, l)
                        } else {
                            let hdc = hwnd.begin_paint(&mut ps);

                            for i in 0..1000 {
                                let x = rand::random::<i32>() % r.right;
                                let y = rand::random::<i32>() % r.bottom;
                                hdc.set_pixel(IVec2 { x, y }, COLORREF(0300));
                            }

                            hwnd.end_paint(&mut ps);

                            hwnd.default_window_procedure(msg, w, l)
                        }
                    }
                    Message::PointerDown(pointer_info)
                    | Message::PointerUpdate(pointer_info)
                    | Message::PointerUp(pointer_info) => {
                        let hdc = HDC::get_device_context(&hwnd);
                        match pointer_info.pointer_type {
                            PointerType::Pen(pen_event) => unsafe {
                                let size = (pen_event.pressure / 10) as i32;
                                hdc.ellipse(
                                    pointer_info.local_position.x - size,
                                    pointer_info.local_position.y - size,
                                    pointer_info.local_position.x + size,
                                    pointer_info.local_position.y + size,
                                );
                            },
                            _ => unsafe {
                                hdc.set_pixel(pointer_info.local_position, COLORREF(0300));
                            },
                        }

                        hdc.release_device_context(&hwnd);

                        hwnd.handled()
                    }
                    Message::Destroy => {
                        println!("WM_DESTROY");
                        unsafe {
                            PostQuitMessage(0);
                        }
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    Message::LeftButtonDown => {
                        println!("WM_LBUTTONDOWN");
                        on_click(hwnd);
                        hwnd.handled()
                    }
                    Message::RightButtonDown => {
                        println!("WM_RBUTTONDOWN");
                        on_right_click(hwnd);
                        hwnd.handled()
                    }
                    Message::LeftButtondDoubleClick => {
                        println!("WM_LBUTTONDBLCLK");
                        hwnd.handled()
                    }
                    Message::Command(command_info) => {
                        match command_info.command {
                            Command::Clicked => {
                                if let Some(click) = command_info.handle.get_user_data::<Callback>()
                                {
                                    click.0(command_info.handle, msg, w, l)
                                } else {
                                    command_info.handle.default_window_procedure(msg, w, l)
                                };
                            }
                            Command::DoubleClick => {
                                println!("Button BN_DBLCLK")
                            }
                            Command::Pushed => {
                                println!("Button BN_PUSHED")
                            }
                            Command::Change => {
                                println!("Edit EN_CHANGE")
                            }
                            _ => {
                                println!("WM_COMMAND WUT? WUT?")
                            }
                        }

                        hwnd.handled()
                    }
                    _ => hwnd.default_window_procedure(msg, w, l),
                }
            })
            .build();

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
            .on_click(move |button| {
                if handle.is_maximized() {
                    handle.restore();
                    button.set_window_text("maximize");
                } else {
                    handle.maximize();
                    button.set_window_text("resotre");
                }
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

        dispatch_thread_events();
    });
}
