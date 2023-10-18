use std::{cell::RefCell, rc::Rc, time::Instant};

use glam::{Affine2, IVec2, Vec2};
use hammer::PointerEventInput;
use leptos_reactive::{SignalGet, SignalGetUntracked, SignalUpdate};
mod hammer;

use windows_reactive::{
    direct_2d::Direct2d,
    hwnd_builder::create_window_handle,
    message_ext::dispatch_thread_events,
    messages::{message_handler, Message},
    window_handle_ext::WindowHandleExt,
    HWND, PAINTSTRUCT,
};

pub fn main() {
    let _ = leptos_reactive::create_scope(leptos_reactive::create_runtime(), |cx| {
        let (camera, set_camera) = leptos_reactive::create_signal(cx, Affine2::IDENTITY);
        let (touches, set_touches) = leptos_reactive::create_signal::<Vec<Vec2>>(cx, Vec::new());
        let (point_center, set_point_center) =
            leptos_reactive::create_signal::<Option<Vec2>>(cx, None);
    });

    let mut zoom_start = Box::new(1.0);
    let mut position_start = Box::new(IVec2::ZERO);
    let mut rotation_start = Box::new(0.0);

    let mut is_pinter_down = Box::new(false);

    let direct2d = Direct2d::new().unwrap();
    // let direct2d = Rc::new(RefCell::new(direct2d));

    let test = "test".to_string();
    let test = Rc::new(RefCell::new(test));

    // leptos_reactive::create_effect(cx, {
    //     let direct2d = direct2d.clone();
    //     move |_| {
    //         println!("did you call me?");
    //         let mut direct2d = direct2d.borrow_mut();
    //         direct2d.camera = camera.get();
    //         direct2d.point_center = point_center.get_untracked();
    //         direct2d.touches = touches.get_untracked();
    //         direct2d.render().unwrap();
    //     }
    // });

    let mut pointer_event_input = PointerEventInput::new();

    let mut counter = Box::new(0);

    let mut builder = create_window_handle()
        .class_name("NativeWindowsGuiWindow")
        .size((500, 500))
        .position((300, 300))
        .text("Basic Window")
        .window()
        .resizable()
        .maximizable()
        .minimizable()
        .always_on_top()
        .visible()
        .on_message({
            let mut direct2d = direct2d;
            let mut is_pinter_down = is_pinter_down;
            move |hwnd, msg, w, l| {
                let message = message_handler(hwnd, msg, w, l);
                match message {
                    Message::Create(_) => {
                        direct2d.set_handle(hwnd);
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    Message::MouseWheel(info) => {
                        // set_position.update(|position| *position = info.local_position);
                        hwnd.handled()
                    }
                    Message::PointerDown(event) => {
                        let input = pointer_event_input.handler(event);
                        *is_pinter_down = true;
                        *position_start = input.center;
                        *zoom_start = input.scale;
                        *rotation_start = input.rotation;
                        *counter = 0;
                        hwnd.handled()
                    }
                    Message::PointerUpdate(event) => {
                        let start_time = Instant::now();
                        let input = pointer_event_input.handler(event);
                        let is_pinter_down = *is_pinter_down;
                        if is_pinter_down {
                            // *counter += 1;
                            // println!("🔴 counter {:?}", counter);
                            // set_camera.update(|camera| {
                            //     let zoom = input.scale - *zoom_start;
                            //     *zoom_start = input.scale;
                            //     let scale = Affine2::from_scale(Vec2::from((zoom, zoom)));
                            //     let position = input.center - *position_start;
                            //     // *position = input.delta.as_ivec2();
                            //     *position_start = input.center;
                            //     let transform = Affine2::from_translation(position.as_vec2());
                            //     let rotation = input.rotation - *rotation_start;
                            //     *rotation_start = input.rotation;
                            //     let rotation = Affine2::from_angle(rotation);
                            //     *camera = *camera * (transform * rotation) //* scale;
                            // });
                            // set_point_center.update(|set_point_center| {
                            //     *set_point_center = Some(input.center.as_vec2());
                            // });
                            let input_touches: Vec<Vec2> =
                                input.pointers_vec.iter().map(|v| v.as_vec2()).collect();
                            // set_touches.update(|touches| {
                            //     *touches = input_touches;
                            // });
                            // let mut direct2d = direct2d.borrow_mut();
                            direct2d.point_center = Some(input.center.as_vec2());
                            direct2d.touches = input_touches;
                            let mut ps = PAINTSTRUCT::default();
                            hwnd.begin_paint(&mut ps);
                            direct2d.render().unwrap();
                            hwnd.end_paint(&mut ps);
                        }
                        let elapsed_time = start_time.elapsed();
                        // println!("Elapsed time: {:?}ms", elapsed_time);
                        // hwnd.handled()
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    Message::PointerUp(event) => {
                        let input = pointer_event_input.handler(event);
                        *is_pinter_down = false;
                        *position_start = IVec2::ZERO;
                        *zoom_start = 1.0;
                        *rotation_start = 0.0;
                        *counter = 0;
                        // set_point_center.update(|point_center| {
                        //     *point_center = None;
                        // });
                        // set_touches.update(|touches| {
                        //     touches.clear();
                        // });
                        direct2d.point_center = None;
                        direct2d.touches.clear();
                        direct2d.render().unwrap();
                        hwnd.handled()
                    }
                    Message::Paint => {
                        let mut ps = PAINTSTRUCT::default();
                        hwnd.begin_paint(&mut ps);
                        direct2d.render().unwrap();
                        hwnd.end_paint(&mut ps);
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    Message::Size => {
                        direct2d.resize_swapchain_bitmap().unwrap();
                        hwnd.handled()
                    }
                    Message::User => hwnd.default_window_procedure(msg, w, l),
                    Message::Destroy => {
                        HWND::post_quit_message();
                        hwnd.handled()
                    }
                    _ => hwnd.default_window_procedure(msg, w, l),
                }
            }
        });

    let window = builder.build();

    dispatch_thread_events();
}
