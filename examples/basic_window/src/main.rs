use leptos_reactive::{SignalGet, SignalUpdate};
use number_into_words::encode;
use windows_reactive::{
    device_context_ext::DeviceContextExt,
    hwnd_builder::{
        create_window_handle, Callback, MouseEvent, PenEvent, PointerId, PointerInfo, PointerType,
        TouchEvent,
    },
    message_ext::dispatch_thread_events,
    param_ext::{LParamExt, ParamExt},
    pre_settings,
    window_handle_ext::WindowHandleExt,
    PostQuitMessage, BN_CLICKED, BN_DBLCLK, BN_PUSHED, COLORREF, EN_CHANGE, HDC, HWND, LRESULT,
    PAINTSTRUCT, PT_MOUSE, PT_PEN, PT_POINTER, PT_TOUCH, PT_TOUCHPAD, WM_CLOSE, WM_COMMAND,
    WM_CREATE, WM_DESTROY, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_PAINT, WM_POINTERDOWN,
    WM_POINTERUP, WM_POINTERUPDATE, WM_RBUTTONDOWN,
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
                match msg {
                    WM_CREATE => {
                        println!("WM_CREATE");
                        hwnd.handled()
                    }
                    WM_CLOSE => {
                        println!("WM_CLOSE");
                        hwnd.hide();
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    WM_PAINT => {
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
                                hdc.set_pixel(x, y, COLORREF(0300));
                            }

                            hwnd.end_paint(&mut ps);

                            hwnd.default_window_procedure(msg, w, l)
                        }
                    }
                    WM_POINTERDOWN | WM_POINTERUPDATE | WM_POINTERUP => {
                        let pointer_id = PointerId::new(w);
                        let pointer_input_type = pointer_id.get_pointer_type();
                        let pointer_info = pointer_id.get_pointer_info();
                        let pointer_info = match pointer_input_type {
                            PT_POINTER => PointerInfo {
                                pointerId: pointer_id,
                                location: hwnd.screen_to_client(&pointer_info.ptPixelLocation),
                                time: pointer_info.dwTime,
                                frameId: pointer_info.frameId,
                                pointerType: PointerType::Pointer,
                            },
                            PT_TOUCH => {
                                let pointer_touch_info = pointer_id.get_pointer_touch_info();

                                PointerInfo {
                                    pointerId: pointer_id,
                                    location: hwnd.screen_to_client(&pointer_info.ptPixelLocation),
                                    time: pointer_info.dwTime,
                                    frameId: pointer_info.frameId,
                                    pointerType: PointerType::Touch(TouchEvent {}),
                                }
                            }
                            PT_PEN => {
                                let pointer_pen_info = pointer_id.get_pointer_pen_info();

                                PointerInfo {
                                    pointerId: pointer_id,
                                    location: hwnd.screen_to_client(&pointer_info.ptPixelLocation),
                                    time: pointer_info.dwTime,
                                    frameId: pointer_info.frameId,
                                    pointerType: PointerType::Pen(PenEvent {
                                        pressure: pointer_pen_info.pressure,
                                        tilt: (pointer_pen_info.tiltX, pointer_pen_info.tiltY),
                                        rotation: pointer_pen_info.rotation,
                                    }),
                                }
                            }
                            PT_MOUSE => PointerInfo {
                                pointerId: pointer_id,
                                location: hwnd.screen_to_client(&pointer_info.ptPixelLocation),
                                time: pointer_info.dwTime,
                                frameId: pointer_info.frameId,
                                pointerType: PointerType::Mouse(MouseEvent {}),
                            },
                            PT_TOUCHPAD => PointerInfo {
                                pointerId: pointer_id,
                                location: hwnd.screen_to_client(&pointer_info.ptPixelLocation),
                                time: pointer_info.dwTime,
                                frameId: pointer_info.frameId,
                                pointerType: PointerType::Touchpad,
                            },
                            _ => {
                                panic!()
                            }
                        };

                        let hdc = HDC::get_device_context(&hwnd);

                        match pointer_info.pointerType {
                            PointerType::Pen(pen_event) => unsafe {
                                let size = (pen_event.pressure / 10) as i32;
                                hdc.ellipse(
                                    pointer_info.location.x - size,
                                    pointer_info.location.y - size,
                                    pointer_info.location.x + size,
                                    pointer_info.location.y + size,
                                );
                            },
                            _ => unsafe {
                                hdc.set_pixel(
                                    pointer_info.location.x,
                                    pointer_info.location.y,
                                    COLORREF(0300),
                                );
                            },
                        }
                        // unsafe { MoveToEx(hdc, 0, 0, None) };
                        // unsafe {
                        //     LineTo(hdc, pointer_info.location.x, pointer_info.location.y)
                        // };
                        hdc.release_device_context(&hwnd);

                        // println!("pointer_info {:?}", pointer_info);
                        // println!("----");

                        hwnd.handled()
                    }
                    WM_DESTROY => {
                        println!("WM_DESTROY");
                        unsafe {
                            PostQuitMessage(0);
                        }
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    WM_LBUTTONDOWN => {
                        println!("WM_LBUTTONDOWN");
                        on_click(hwnd);
                        hwnd.handled()
                    }
                    WM_RBUTTONDOWN => {
                        println!("WM_RBUTTONDOWN");
                        on_right_click(hwnd);
                        hwnd.handled()
                    }
                    WM_LBUTTONDBLCLK => {
                        println!("WM_LBUTTONDBLCLK");
                        hwnd.handled()
                    }
                    WM_COMMAND => {
                        let child_handle = l.get_child_handle();
                        let message = w.get_hiword();

                        let class_name = child_handle.get_class_name();

                        match &class_name as &str {
                            "Button" => match message {
                                BN_CLICKED => {
                                    if let Some(click) = child_handle.get_user_data::<Callback>() {
                                        click.0(child_handle, msg, w, l)
                                    } else {
                                        child_handle.default_window_procedure(msg, w, l)
                                    };
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
                                println!("WM_COMMAND WUT? WUT?")
                            }
                        };

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
