use leptos_reactive::{
    create_runtime, create_scope, create_signal, SignalGet, SignalGetUntracked, SignalUpdate,
};
use std::{cell::RefCell, rc::Rc};
use windows_reactive::{
    direct_2d::Direct2d,
    droppable::{self, Droppable},
    game::Game,
    hwnd_builder::create_window_handle,
    message_ext::dispatch_thread_events,
    messages::{message_handler, Message},
    test::Test,
    window_handle_ext::WindowHandleExt,
    HWND, PAINTSTRUCT,
};

pub fn main() {
    let _ = create_scope(create_runtime(), |cx| {
        let (is_pinter_down, set_is_pinter_down) = create_signal(cx, false);
    });
    // let game = Game::initialize();
    // let test = Box::new("test");
    // let mut counter = Box::new(0);
    let mut is_pinter_down = Box::new(false);
    let mut direct2d = Direct2d::new().unwrap();
    // let direct2d = Rc::new(RefCell::new(direct2d));
    let mut ones = Box::new(false);
    let mut droppable = Box::new(Droppable::new("fuck this shit"));

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
            move |hwnd, msg, w, l| {
                let message = message_handler(hwnd, msg, w, l);
                match message {
                    Message::Create(_) => {
                        direct2d.set_handle(hwnd);
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    Message::PointerUp(_) => {
                        *is_pinter_down = false;
                        hwnd.handled()
                    }
                    Message::PointerDown(_) => {
                        *is_pinter_down = true;

                        hwnd.handled()
                    }
                    Message::PointerUpdate(_) => {
                        droppable.tick();
                        direct2d.render().unwrap();
                        // if !*ones {
                        //     let mut direct2d = direct2d.borrow_mut();
                        //     // drop(direct2d);
                        //     println!("🟢 2 PointerUpdate");
                        //     // *ones = true
                        // }
                        hwnd.handled()
                    }

                    Message::Paint => {
                        // let mut direct2d = direct2d.borrow_mut();
                        hwnd.default_window_procedure(msg, w, l)
                    }
                    Message::Size => hwnd.handled(),
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
