use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        BS_FLAT, BS_PUSHBUTTON, CW_USEDEFAULT, WINDOW_EX_STYLE, WINDOW_STYLE, WS_BORDER,
        WS_CAPTION, WS_CHILD, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW,
        WS_EX_LAYERED, WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
        WS_EX_WINDOWEDGE, WS_MAXIMIZE, WS_MAXIMIZEBOX, WS_MINIMIZE, WS_MINIMIZEBOX,
        WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SIZEBOX, WS_SYSMENU, WS_VISIBLE,
    },
};

use crate::{
    user_data_ext::{Callback, UserDataExt},
    window_handle_ext::WindowHandleExt,
};

#[derive(Default)]
pub struct HwndBuilder {
    class_name: String,
    text: Option<String>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
    click_callback: Option<Box<dyn Fn(HWND)>>,
    parent: Option<HWND>,
    style: WINDOW_STYLE,
    ex_style: WINDOW_EX_STYLE,
}

impl HwndBuilder {
    pub fn parent<'a>(mut self, parent: &'a HWND) -> HwndBuilder {
        self.parent = Some(*parent);
        self
    }

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

    pub fn on_click(mut self, f: impl Fn(HWND) + 'static) -> HwndBuilder {
        self.click_callback = Some(Box::new(f));
        self
    }

    pub fn resizable(mut self) -> HwndBuilder {
        self.style |= WS_SIZEBOX;
        self
    }

    pub fn minimizable(mut self) -> HwndBuilder {
        self.style |= WS_MINIMIZEBOX;
        self
    }

    pub fn maximizable(mut self) -> HwndBuilder {
        self.style |= WS_MAXIMIZEBOX;
        self
    }

    pub fn visible(mut self) -> HwndBuilder {
        self.style |= WS_VISIBLE;
        self
    }

    pub fn push_button(mut self) -> HwndBuilder {
        self.style |= WINDOW_STYLE(BS_PUSHBUTTON as u32);
        self
    }

    pub fn flat(mut self) -> HwndBuilder {
        self.style |= WINDOW_STYLE(BS_FLAT as u32);
        self
    }

    pub fn child(mut self) -> HwndBuilder {
        self.style |= WS_CHILD;
        self
    }

    pub fn on_taskbar(mut self) -> HwndBuilder {
        self.ex_style |= WS_EX_APPWINDOW;
        self
    }

    pub fn always_on_top(mut self) -> HwndBuilder {
        self.ex_style |= WS_EX_TOPMOST;
        self
    }

    pub fn no_back_buffer(mut self) -> HwndBuilder {
        self.ex_style |= WS_EX_NOREDIRECTIONBITMAP;
        self
    }

    pub fn popup(mut self) -> HwndBuilder {
        self.style |= WS_POPUP;
        self
    }

    pub fn minimized(mut self) -> HwndBuilder {
        self.style |= WS_MINIMIZE;
        self
    }

    pub fn maximized(mut self) -> HwndBuilder {
        self.style |= WS_MAXIMIZE;
        self
    }

    pub fn ignore_cursor_event(mut self) -> HwndBuilder {
        self.ex_style |= WS_EX_TRANSPARENT | WS_EX_LAYERED;
        self
    }

    pub fn fullscreen(mut self) -> HwndBuilder {
        self.style &= !WS_OVERLAPPEDWINDOW;
        self
    }

    ///
    /// Should go at the beginning
    pub fn window(mut self) -> HwndBuilder {
        self.style = WS_CAPTION | WS_BORDER | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU;
        self.ex_style = WS_EX_WINDOWEDGE | WS_EX_ACCEPTFILES;
        self
    }

    pub fn build(self) -> HWND {
        let class_name = &self.class_name;
        let (width, height) = self.size.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let (x, y) = self.pos.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let window_name = &self.text.unwrap_or("".to_string());
        let parent = self.parent.unwrap_or(HWND(0));

        let handle = {
            let handle = HWND::create_window(
                self.ex_style,
                class_name,
                window_name,
                self.style,
                x,
                y,
                width,
                height,
                parent,
                None,
                Some("Test 123 qwe"),
            );

            if let Some(f) = self.click_callback {
                let callback = Callback::new(f);
                handle.add(callback);
            }

            handle
        };

        handle
    }
}

pub fn create_window_handle() -> HwndBuilder {
    HwndBuilder::default()
}
