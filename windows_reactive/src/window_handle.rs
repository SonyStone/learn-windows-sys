use signal::Runtime;
use windows::{
    core::Result,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            IsWindow, ShowWindow, CW_USEDEFAULT, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE,
        },
    },
};
use windows_stuff::window_state::WindowFlags;

use crate::{user_data_ext::Callback, window_handle_ext::WindowHandleExt};

#[derive(Default)]
pub struct HwndBuilder {
    class_name: String,
    text: Option<String>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
}

impl HwndBuilder {
    pub fn class_name<'a>(mut self, name: &'a str) -> HwndBuilder {
        self.class_name = name.to_string();
        self
    }

    pub fn text<'a>(mut self, text: &'a str) -> HwndBuilder {
        self.text = Some(text.to_string());
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> HwndBuilder {
        self.size = Some(size);
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> HwndBuilder {
        self.pos = Some(pos);
        self
    }

    pub fn build(self) -> WindowHandle {
        let class_name = &self.class_name;
        let (nwidth, nheight) = self.size.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let (x, y) = self.pos.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));

        let mut window_flags = WindowFlags::empty();
        window_flags.set(WindowFlags::RESIZABLE, true);
        window_flags.set(WindowFlags::MINIMIZABLE, true);
        window_flags.set(WindowFlags::MAXIMIZABLE, true);
        window_flags.set(WindowFlags::VISIBLE, true);
        let (style, ex_style) = window_flags.to_window_styles();

        let cx: &'static Runtime = Box::leak(Box::default());
        let count = cx.create_signal(0);

        let callback = Callback::new(move || {
            let current = count.get();
            count.set(current + 1);
        });

        let handle = {
            let handle = HWND::create_window(
                ex_style,
                "NativeWindowsGuiWindow",
                "Learn to Program Windows",
                style,
                x,
                y,
                nwidth,
                nheight,
                None,
                None,
                Some("Test 123 qwe"),
            );

            cx.create_effect(move || {
                let count = &count.get().to_string();
                let text = format!("Hello me! {}", count);
                println!("{}", text);
                handle.set_window_text(text.as_str());
            });

            // handle.add(callback);

            WindowHandle(handle)
        };

        handle
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct WindowHandle(HWND);

impl WindowHandle {
    pub fn is_window(&self) -> Result<()> {
        let result = unsafe { IsWindow(self.0) };
        result.ok()
    }

    // + ещё 15 различных состояний.
    pub fn minimize(&self) {
        unsafe {
            ShowWindow(self.0, SW_MINIMIZE);
        }
    }

    pub fn maximize(&self) {
        unsafe {
            ShowWindow(self.0, SW_MAXIMIZE);
        }
    }

    pub fn restore(&self) {
        unsafe {
            ShowWindow(self.0, SW_RESTORE);
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.0
    }
}

pub fn create_window_handle() -> HwndBuilder {
    HwndBuilder::default()
}
