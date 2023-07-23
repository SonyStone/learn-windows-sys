use leptos::*;
use signal::*;
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};
use web_sys::{EventTarget, KeyboardEvent, MouseEvent};

fn main() {
    console_error_panic_hook::set_once();

    let body = document().body().unwrap();
    let p = document().create_element("p").unwrap();
    p.set_text_content(Some("No text yet"));
    let double = document().create_element("p").unwrap();
    double.set_text_content(Some("No text yet"));
    let inc = document().create_element("button").unwrap();
    inc.set_text_content(Some("+1"));
    let dec = document().create_element("button").unwrap();
    dec.set_text_content(Some("-1"));

    let input = document().create_element("input").unwrap();

    body.append_child(&inc).unwrap();
    body.append_child(&p).unwrap();
    body.append_child(&double).unwrap();
    body.append_child(&input).unwrap();
    body.append_child(&dec).unwrap();

    let cx: &'static Runtime = Box::leak(Box::default());
    let count = cx.create_signal(0);
    let double_counts = move || count.get() as f64 * 2.3;

    cx.create_effect(move || p.set_text_content(Some(&count.get().to_string())));

    cx.create_effect(move || double.set_text_content(Some(&double_counts().to_string())));

    add_event_listener(&input, "input", move |_: KeyboardEvent| {
        log!("+1");
        let current = count.get();
        count.set(current + 1);
    });

    add_event_listener(&inc, "click", move |_: MouseEvent| {
        log!("+1");
        let current = count.get();
        count.set(current + 1);
    });

    add_event_listener(&dec, "click", move |_: MouseEvent| {
        log!("-1");
        let current = count.get();
        count.set(current - 1);
    });
}

fn add_event_listener<E>(element: &EventTarget, event_name: &str, cb: impl FnMut(E) + 'static)
where
    E: FromWasmAbi + 'static,
{
    let cb = Closure::wrap(Box::new(cb) as Box<dyn FnMut(E)>).into_js_value();

    element
        .add_event_listener_with_callback(event_name, cb.unchecked_ref())
        .unwrap();
}
